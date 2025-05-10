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
    pub gravity: Vector3<f32>,
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
            .translation(Vector3::new(0.0, 0.0, 0.0))
            .build();
        let ball_collider = ColliderBuilder::ball(0.1).build();
        colliders.insert(ball_collider);
        let ball_handle = world.insert(ball);

        println!("Outputting physics world.");

        //This is outputting the physics world as it's return variable using the struct.
        PhysicsWorld {
            world,
            colliders,
            physics_pipeline,
            gravity: Vector3::new(0.0, 0.0, 0.0), //No gravity as all the directions are controlled programatically.
            //Could potentially add gravity later though.
            player_map: HashMap::new(),
            collider_map: HashMap::new(),
            ball_handle,
        }



    }

    pub fn step(&mut self, dt: f32) {
        let gravity = self.gravity;
        let integration_parameters = IntegrationParameters {
            dt: dt,
            ..Default::default()
        };
        let mut solver = ImpulseSolver::new();
        self.physics_pipeline.step(&gravity, &integration_parameters, &mut solver);
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