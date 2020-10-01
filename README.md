# ECS
## Systems
DrawSystem - Draws entities to the screen (thread local)
AnimationSystem - Updates the animation frame for animated components
TimeSystem - Computes the delta time step
ControlSystem - Checks keyboard input and updates entity velocity
PhysicsSystem - Integrates entity position based on velocity and delta time

## Components
Position - x, y, z position in world coordinates
Velocity - x, y, z velocity in world coordinates
Animation - Animation speed and frames
Draw - Shader and buffer information
Controlled - If an entity is updated by InputState

## Resources
DeltaTime - Elapsed time since last run
InputState - Up, Down, Left, Right, Action Keypress states