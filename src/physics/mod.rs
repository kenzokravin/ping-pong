//This is the physics world file.
//Within this file is where all the physics calcs can be made and all the required data is organised.
//


use rapier3d::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;


// Simple physics world struct to hold the rapier world
//This is used to create our own world in the main() function.
pub struct PhysicsWorld {
    pub world: RigidBodySet,
    pub colliders: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: Box<dyn PhysicsHooks + Send + Sync>,
    pub event_handler: Box<dyn EventHandler + Send + Sync>,
    pub gravity: Vector<f32>,
    pub player_map: HashMap<Uuid,RigidBodyHandle>,
    pub collider_map: HashMap<ColliderHandle, Uuid>, //Reverse lookup for collision detect.
    pub ball_handle: RigidBodyHandle,
}

impl PhysicsWorld {
    pub fn new() -> Self {


        let mut world = RigidBodySet::new(); //creating empty collections.
        let mut colliders = ColliderSet::new();

        let mut physics_pipeline = PhysicsPipeline::new();

        // You could initialize the world with some simple objects, like paddles and ball
        let ball = RigidBodyBuilder::dynamic()
            .translation(Vector::new(0.0, 0.0, 0.0))
            .build();
        let ball_collider = ColliderBuilder::ball(0.1).build();
        colliders.insert(ball_collider);
        let ball_handle = world.insert(ball);


        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        let physics_hooks = Box::new(());
        let event_handler = Box::new(());

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

        let island_manager = self.island_manager;
        let broad_phase = self.broad_phase;
        let narrow_phase = self.narrow_phase;
        let rigid_body_set = self.world;
        let collider_set = self.colliders;
        let impulse_joint_set = self.impulse_joint_set;
        let multibody_joint_set = self.multibody_joint_set;
        let ccd_solver = self.ccd_solver;
        let query_pipeline = self.query_pipeline;


        //These are wrong, need to use something different.
        let physics_hooks = self.physics_hooks;
        let event_handler = self.event_handler;


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
            &physics_hooks,
            &event_handler,
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
        //tracking player colliders
        self.collider_map.insert(player_collider_handle,player_id);

        // You could store paddle_handle in a map if you want to track per-player paddles
    }

}