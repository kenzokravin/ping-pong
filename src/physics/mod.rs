//This is the physics world file.
//Within this file is where all the physics calcs can be made and all the required data is organised.
//Within each instance is a list of the players, their physics bodies, the ball, table.


use std::sync::Arc;
use tokio::sync::Mutex;

use rapier3d::prelude::*;
use crate::nalgebra::Vector3;
use crate::nalgebra::distance;
use std::collections::HashMap;
use uuid::Uuid;

mod game_state;
use game_state::Timer;


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
    pub move_intents: HashMap<Uuid,Vector3<f64>>,
    pub ball_handle: RigidBodyHandle,
    pub player_order_map: HashMap<Uuid, i32>,
    pub player_shot_timer: HashMap<Uuid,Arc<Mutex<Timer>>>
}

impl PhysicsWorld {
    pub fn new() -> Self {


        let mut world = RigidBodySet::new(); //creating empty collections.
        let mut colliders = ColliderSet::new();

        let physics_pipeline = PhysicsPipeline::new();

        // Init ball to be used.
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
            move_intents: HashMap::new(),
            ball_handle,
            player_order_map: HashMap::new(),
            player_shot_timer: HashMap::new(),
        }



    }

    pub fn step(&mut self, dt: f32) {

        //Creating/Accessing all the step parameters.

        let gravity = self.gravity;
        let integration_parameters = IntegrationParameters {
            dt: dt,
            ..Default::default()
        };

        let mut island_manager = &mut self.island_manager;
        let mut broad_phase = &mut self.broad_phase;
        let mut narrow_phase = &mut self.narrow_phase;
        let mut rigid_body_set = &mut self.world;
        let mut collider_set = &mut self.colliders;
        let mut impulse_joint_set = &mut self.impulse_joint_set;
        let mut multibody_joint_set = &mut self.multibody_joint_set;
        let mut ccd_solver =  &mut self.ccd_solver;
        let mut query_pipeline = &mut self.query_pipeline;


        // if !self.move_intents.is_empty() {
        //     for (player_id, pos) in self.move_intents.drain() {
        //         self.set_player_position(player_id, pos.x as f64, pos.y as f64, pos.z as f64);
        //     }
        // }

        //The issue that stems from the phys world not updating is that we were creating a copy of the world, updating the original and then
        //performing the physics step, so no updates were actually occurring.
        //FIXED: Used the called rigid_body_set. Makes so much sense, this was idiotic. Rust only allows 1 editable ref to data at a time.

        if !self.move_intents.is_empty() {
        let intents = std::mem::take(&mut self.move_intents);
        for (player_id, pos) in intents { //For each movement intent that is queued.
            if let Some(&body_handle) = self.player_map.get(&player_id) //Access the rigid body handle.
            {

                if let Some(rigid_body) = rigid_body_set.get_mut(body_handle) {
                    assert_eq!(rigid_body.body_type(), RigidBodyType::KinematicPositionBased);

                    rigid_body.set_enabled(true);
                    rigid_body.set_next_kinematic_translation(vector![pos.x as f32,pos.y as f32,pos.z as f32]);

                    let position = rigid_body.translation();
                  //  println!("Player: {} position in world space: x = {}, y = {}, z = {}", player_id, position.x, position.y, position.z);
                }
            }
        }
        }

        //These are wrong, need to use something different.
        //let physics_hooks = &self.physics_hooks;

        let hooks_ref: &mut dyn rapier3d::pipeline::PhysicsHooks = &mut *self.physics_hooks;
        let event_ref: &mut dyn rapier3d::pipeline::EventHandler = &mut *self.event_handler;
        //let event_handler = &self.event_handler;
        //let physics_hooks = self.physics_hooks.clone();
        //let event_handler = self.event_handler.clone();


        //let mut solver = ImpulseSolver::new();
        //self.physics_pipeline.step(&gravity, &integration_parameters, &mut solver);


        //This is where we could allow player movement to be changed.
        //Use the move_intents hashmap, and loop through, addressing the player id and updating it's movement.
        //If we go non-removal from hashmap route, the first player will always be updated first 
        // (but if they are all updated in step then does it matter?)
        // Though if the player stops moving, it will continually update their position when it doesn't have to...
        // So perhaps, for performance of server, it is better to remove once the job is complete.

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

        //println!("Physics world Step!");

    }

    pub fn add_player(&mut self, player_id: Uuid)   {
        //&mut self just allows the function to reference it's var (in this case being physics world)
        //This allows us to make changes to the collections such as world, and colliders etc
        //player_id is just the unique player id and it's type.

        //Creating and inserting player rigidBody to rigidBodySet (world).
        let player_body = RigidBodyBuilder::kinematic_position_based()
            .translation(vector![0.0, 0.0, 0.0]) // Starting position
            .build();
        let player_body_handle = self.world.insert(player_body);

        //Creating bat collider, cylinder in shape. This is what will follow the player mouse and communicate "hits".
   
        let player_collider = ColliderBuilder::cylinder(0.05, 0.75).rotation(vector![0.0, 0.0, std::f32::consts::FRAC_PI_2]).build();
        //Adding collider to collider set.
        let player_collider_handle = self.colliders.insert_with_parent(player_collider, player_body_handle, &mut self.world);

        let mut player_index_num = 0;

        for (player_id, p_body_handle) in &self.player_map {
            player_index_num += 1; //Determining index of player_order using world rigid_bodies.
            println!("Adding to index: {}", player_index_num);
        }

        //Tracking player_id and their player order, which will be used to set sides of board.
        self.player_order_map.insert(player_id,player_index_num);

        //Tracking player bodies using player ID, used to remove player bodies from physics world. (from rigid body set)
        self.player_map.insert(player_id,player_body_handle);
        //tracking player colliders, this is used for determining which player is involved in collision based of the collider handle.
        self.collider_map.insert(player_collider_handle,player_id);

        

        //Tracking the player_collider handles, this enables us to remove the collider_map entry for the player_id when disconnecting. 
        self.player_collider_map.insert(player_id,player_collider_handle);

        println!("Player added to physics world: {}",player_id);

        // You could store paddle_handle in a map if you want to track per-player paddles
    }

    pub fn remove_player(&mut self, player_id:Uuid) {

        //Accessing player_body_handle using player_id
        let player_body_handle = self.player_map.get(&player_id).copied().unwrap();

        //Accessing player_body_handle using player_id, check exist, Then remove from rigidBodySet of physics world.
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

            // Remove tracking info in player_map (for rigid bodies)
            self.player_map.remove(&player_id);

            //Remove info for the player_index.
            self.player_order_map.remove(&player_id);

            //Accessing collider handle, using result to remove from collider map, then removing from the joining map (player_collider_map)
            let player_collider_handle = self.player_collider_map.get(&player_id).copied().unwrap();
            self.collider_map.remove(&player_collider_handle);
            self.player_collider_map.remove(&player_id);
       
           //Prints all colliders left in collider_map.
            for (key, value) in &self.collider_map {
                    println!("Key: {:?}, Value: {:?}", key, value);
            }
    
            //Print if successful.
            println!("Player removed from physics world: {}", player_id);
        } else {
            println!("Tried to remove non-existent player: {}", player_id);
        }


    }

    pub fn set_player_position(&mut self, player_id: Uuid, dx: f64, dy: f64, dz: f64) {

         if let Some(&body_handle) = self.player_map.get(&player_id) 
        {

            let mut rigid_body = self.world.get_mut(body_handle).unwrap();
            assert_eq!(rigid_body.body_type(), RigidBodyType::KinematicPositionBased);

            rigid_body.set_enabled(true);
            rigid_body.set_next_kinematic_translation(vector![dx as f32,dy as f32,dz as f32]);

            let position = rigid_body.translation();
            //println!("Player position in world space: x = {}, y = {}, z = {}", position.x, position.y, position.z);


           // println!("Movement set.");
           //Adding player insert to hashmap so it can be processed in the physics world step function.
          //  self.move_intents.insert(player_id,vector![dx as f32,dy as f32,dz as f32]); 

        }


    }

    pub fn add_move_to_queue(&mut self, player_id: Uuid, dx: f64, dy: f64, mut dz: f64) {

        //Accessing rigid body from player.
        if let Some(&body_handle) = self.player_map.get(&player_id) 
        {

            let mut rigid_body = self.world.get_mut(body_handle).unwrap();
            
            assert_eq!(rigid_body.body_type(), RigidBodyType::KinematicPositionBased);

            if let Some(&player_index_num) = self.player_order_map.get(&player_id) { //used to verify position in world space.

                if player_index_num == 0 {

                    dz = 9.0; //ensuring that the user is on a z level of 9.
        
                } else if player_index_num == 1 {

                    if dz < 0.0 && dz > -9.001 && dz < -8.999 { //ensuring player order are opposite in physics world.
                               // println!("dz is within the target negative range");
                    } else {
                        dz = 9.0;
                        dz *= -1.0; //flipping the user so that they are on opposing sides.
                    }


                   // println!("Flipping User: {} for dz: {}",player_id,dz);

                }


            }
            //rigid_body.set_enabled(true);
           // rigid_body.set_next_kinematic_translation(vector![dx as f32,dy as f32,dz as f32]);
           //Adding player insert to hashmap so it can be processed in the physics world step function.
            self.move_intents.insert(player_id,vector![dx as f64,dy as f64,dz as f64]); 
        }

    }

    pub fn get_player_number(&mut self, player_id: Uuid) -> i32 {
        match self.player_order_map.get(&player_id) {
            Some(&index) => index, //return index value.
            None => -1, // returning an impossible order value if the hash doesn't exist yet.
        }
    }

    pub fn player_hit(&mut self, player_id:Uuid) { //Called when player hits "mouse down" to start hit.

        if let Some(&body_handle) = self.player_map.get(&player_id) 
        {

            //When creating timer, we must verify that there is no existing timer already for an id.
            //This ensures that the user cannot spam timer creation and destroy the server.

            let timer = Arc::new(Mutex::new(Timer::new(3)));
            self.player_shot_timer.insert(player_id,timer);

            println!("Hit timer started.");

        }


    }

    pub async fn player_hit_exec(&mut self, player_id:Uuid) {

        println!("Hit timer started.");
        if let Some(&body_handle) = self.player_map.get(&player_id) 
        {

            if let Some(timer) = self.player_shot_timer.get(&player_id) {
                println!("Timer is here");
                let is_done = timer.lock().await.timer_value().as_millis(); //Retrieving timer value as millis.
                
                println!("{:?} shot time", is_done);
                
            }

            let mut rigid_body = self.world.get_mut(body_handle).unwrap(); //Getting rigidBody of player

            let mut p_position = rigid_body.position(); //retrieving position of hit location.

            let p_pos = Point::from(p_position.translation.vector); //Getting the point from the position.

            let ball_handle = self.ball_handle; //Accessing ball handle.

           // let ball_rb = self.world.get(ball_handle);
           let mut ball_rb = self.world.get_mut(ball_handle).unwrap();

            if let Some(ball_rb) = self.world.get(ball_handle) {

                let mut b_position = ball_rb.position();

                let b_pos = Point::from(b_position.translation.vector);

                let dist = distance(&p_pos,&b_pos); //finding distance.
                println!("hit dist from bat and ball = {}",dist);
            }


            if let Some(timer) = self.player_shot_timer.get(&player_id) { //Removing timer entry for player to prevent duplicates.
                self.player_shot_timer.remove(&player_id);
                println!("Removed timer for player.");
            }


        
        //println!("hit_position = {}",p_position);

        }

    }

    // pub fn distance(start: Vector3<T>, other: Vector3<T>) -> f32 {
    //     let dx = other.x - start.x;
    //     let dy = other.y - start.y;
    //     let dz = other.z - start.z;
    //     (dx * dx + dy * dy + dz * dz).sqrt()
    // }

    //Used to lerp between 2 vector3 values/positions over time t.
    pub fn lerp_vector3(&mut self, start_vec : Vector3<f64>, end_vec:Vector3<f64>, t : f64) -> Vector3<f64> { 

          start_vec + (end_vec - start_vec) * t

        // let mut return_vector = Vector3::new(0.0,0.0,0.0);

        // return_vector[0] = lerp_two_vals(start_vec[0],end_vec[0],t);
        // return_vector[1] = lerp_two_vals(start_vec[1],end_vec[1],t);
        // return_vector[2] = lerp_two_vals(start_vec[2],end_vec[2],t);

        // return_vector

        // return {
        //     x: lerp(start.x, end.x, t),
        //     y: lerp(start.y, end.y, t),
        //     z: lerp(start.z, end.z, t)
        // };
        // }

    }

    //Used to lerp between two values over time, t.
    pub fn lerp_two_vals(a : f64, b : f64, t: f64) -> f64 {
        a + (b - a) * t
    }

 


}

