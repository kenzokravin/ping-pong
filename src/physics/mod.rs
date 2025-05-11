//This is the physics world file.
//Within this file is where all the physics calcs can be made and all the required data is organised.
//


use rapier3d::prelude::*;
use crate::nalgebra::Vector3;
use std::collections::HashMap;
use uuid::Uuid;


// Simple physics world struct to hold the rapier world
//This is used to create our own world in the main() function.
pub struct PhysicsWorld {
    pub world: RigidBodySet,
    pub colliders: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    //pub physics_hooks: dyn PhysicsHooks,
    //pub event_handler: dyn EventHandler,
    pub physics_hooks: Box<dyn PhysicsHooks + Send + Sync>,
    pub event_handler: Box<dyn EventHandler + Send + Sync>,
    pub gravity: Vector<f32>,
    pub player_map: HashMap<Uuid,RigidBodyHandle>,
    pub collider_map: HashMap<ColliderHandle, Uuid>, 
    pub player_collider_map: HashMap<Uuid, ColliderHandle>,//Reverse lookup for collision detect.
    pub ball_handle: RigidBodyHandle,
}

impl PhysicsWorld {
    pub fn new() -> Self {


        let mut world = RigidBodySet::new(); //creating empty collections.
        let mut colliders = ColliderSet::new();

        let physics_pipeline = PhysicsPipeline::new();

        // You could initialize the world with some simple objects, like paddles and ball
        let ball = RigidBodyBuilder::dynamic()
            .translation(Vector3::new(0.0, 0.0, 0.0))
            .build();
        let ball_collider = ColliderBuilder::ball(0.1).build();
        colliders.insert(ball_collider);
        let ball_handle = world.insert(ball);


        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = Box::new(());
        let event_handler = Box::new(());

        //let physics_hooks = <dyn PhysicsHooks>::new();
        //let event_handler = <dyn EventHandler>::new();

        println!("Outputting physics world.");

        //This is outputting the physics world as it's return variable using the struct.
        PhysicsWorld {
            world,
            colliders,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            gravity: Vector3::new(0.0, 0.0, 0.0), //No gravity as all the directions are controlled programatically.
            //Could potentially add gravity later though.
            player_map: HashMap::new(),
            collider_map: HashMap::new(),
            player_collider_map: HashMap::new(),
            ball_handle,
        }



    }

    pub fn step(&mut self, dt: f32) {

        //Creating/Accessing all the step parameters.

        let gravity = self.gravity;
        let integration_parameters = IntegrationParameters {
            dt: dt,
            ..Default::default()
        };

        let mut island_manager = self.island_manager.clone();
        let mut broad_phase = self.broad_phase.clone();
        let mut narrow_phase = self.narrow_phase.clone();
        let mut rigid_body_set = self.world.clone();
        let mut collider_set = self.colliders.clone();
        let mut impulse_joint_set = self.impulse_joint_set.clone();
        let mut multibody_joint_set = self.multibody_joint_set.clone();
        let mut ccd_solver = self.ccd_solver.clone();
        let mut query_pipeline = self.query_pipeline.clone();


        //These are wrong, need to use something different.
        //let physics_hooks = &self.physics_hooks;

        let hooks_ref: &mut dyn rapier3d::pipeline::PhysicsHooks = &mut *self.physics_hooks;
        let event_ref: &mut dyn rapier3d::pipeline::EventHandler = &mut *self.event_handler;
        //let event_handler = &self.event_handler;
        //let physics_hooks = self.physics_hooks.clone();
        //let event_handler = self.event_handler.clone();


        //let mut solver = ImpulseSolver::new();
        //self.physics_pipeline.step(&gravity, &integration_parameters, &mut solver);

        self.physics_pipeline.step(
            &gravity, 
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            hooks_ref,
            event_ref,
        );
    }

    pub fn add_player(&mut self, player_id: Uuid)   {
        //&mut self just allows the function to reference it's var (in this case being physics world)
        //This allows us to make changes to the collections such as world, and colliders etc
        //player_id is just the unique player id and it's type.

        let player_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![0.0, 0.0, 0.0]) // Starting position
            .build();
        let player_body_handle = self.world.insert(player_body);

        let player_collider = ColliderBuilder::cylinder(0.05, 0.5).build();
        let player_collider_handle = self.colliders.insert_with_parent(player_collider, player_body_handle, &mut self.world);



        //tracking player bodies
        self.player_map.insert(player_id,player_body_handle);
        //tracking player colliders, this is used for determining which player is involved in collision based of the collider handle.
        self.collider_map.insert(player_collider_handle,player_id);

        self.player_collider_map.insert(player_id,player_collider_handle);

        println!("Player added to physics world: {}",player_id);

        // You could store paddle_handle in a map if you want to track per-player paddles
    }

    pub fn remove_player(&mut self, player_id:Uuid) {

        let player_body_handle = self.player_map.get(&player_id).copied().unwrap();

        if let Some(&body_handle) = self.player_map.get(&player_id) 
        {
            // Remove the rigid body and associated colliders
            let colliders_removed = self.world.remove(
                body_handle,
                &mut self.island_manager,
                &mut self.colliders,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true, // auto-remove attached colliders
            );

            // Remove tracking info
            self.player_map.remove(&player_id);

            //must remove tracking info from collider map.
        
            let player_collider_handle = self.player_collider_map.get(&player_id).copied().unwrap();
            self.collider_map.remove(&player_collider_handle);
            self.player_collider_map.remove(&player_id);
       
           
            for (key, value) in &self.collider_map {
                    println!("Key: {:?}, Value: {:?}", key, value);
            }
    

            println!("Player removed from physics world: {}", player_id);
        } else {
            println!("Tried to remove non-existent player: {}", player_id);
        }


    }

 


}

