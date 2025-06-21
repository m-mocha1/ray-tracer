# 📘 Ray Tracer Documentation  
Language: Rust  
Project: Ray Tracing Engine  

![img1](https://github.com/user-attachments/assets/5e783243-96a4-49ca-a471-fcab9eef6ef8)

---


## 📌 Introduction

This project is a basic ray tracer implemented in Rust. A ray tracer is a rendering technique for generating realistic images by simulating the way rays of light interact with objects in a scene.

This engine creates .ppm image files showing 3D scenes with lighting, shadows, and reflection using fundamental geometric objects like spheres, cubes, planes, and cylinders.

![img3](https://github.com/user-attachments/assets/131096c6-4011-47b3-aa14-d3dcb49d0bb6)

---

## 🧠 How the Project Works

The ray tracer works by casting rays from a virtual camera into a scene filled with 3D objects. For each ray:

- If it hits an object, the program calculates how light would interact with it.
- It considers color, brightness, and direction to determine the pixel color.
- The program loops through every pixel and writes the color to an image file.

---

## ⭐ Project Features

- Support for multiple geometric shapes: sphere, cube, plane, cylinder.
- Basic lighting and shadow system.
- Adjustable camera position and field of view.
- Reflections (bonus feature).
- Outputs .ppm images.
- Written entirely in safe Rust.

---

## 📁 Project Structure


src/
├── main.rs            // Main entry and image renderer
├── camera.rs          // Camera control and ray generation
├── cube.rs            // Cube object definition
├── cylinder.rs        // Cylinder object
├── hittable.rs        // Hittable trait for any object
├── hittable_list.rs   // Scene object manager
├── ray.rs             // Ray definition
├── sphere.rs          // Sphere object
└── vec3.rs            // 3D vector math and operations


---

## 🧩 Key Components

- *main.rs*: Scene setup and loop that renders the image.
- *camera.rs*: Defines the camera and ray directions per pixel.
- *vec3.rs*: Contains vector math used throughout (dot product, normalization, etc).
- *hittable.rs / hittable_list.rs*: Abstracts the idea of objects that can be hit by a ray.
- *sphere.rs / cube.rs / cylinder.rs / plane.rs*: Each implements the hittable trait.

---

## 🧮 Math & Vector Operations

The core of ray tracing uses:

- Vector math: dot product, cross product, normalization, reflection.
- Ray equations:  
  P(t) = A + t * B  
  where A is the origin and B is the direction.

- Sphere hit logic:  
  dot((P(t) - center), (P(t) - center)) = radius²

- Reflection formula:  
  R = D - 2 * dot(D, N) * N  
  where D is the ray direction, N is the normal.

---

## 🔦 Ray Logic

In ray tracing:

- A ray is a line starting from a point going infinitely in a direction.
- The ray hits an object if it satisfies a geometric equation.
- If an object is hit, the color is calculated based on the normal vector and light source.
- Recursive calls simulate bouncing and shadows.

---

## 🧵 Functions Breakdown

### main.rs

- color(ray, world, depth): Calculates color recursively based on intersections.
- main(): Builds scene, loops through image pixels, generates rays and calls color().

### vec3.rs

- dot(), length(), normalize(), cross(), random_in_unit_sphere(): Basic vector math functions.

### ray.rs

- origin(), direction(), point_at_parameter(t): Define and use rays.

### camera.rs

- Camera::new(): Creates camera based on position and look direction.
- get_ray(u, v): Returns ray from camera through pixel coordinate.

![img2](https://github.com/user-attachments/assets/1b393273-0e5a-4654-8e00-9edec68c4db8)

### sphere.rs

- hit(ray, t_min, t_max): Returns hit record if the ray hits the sphere.

### cube.rs / cylinder.rs / plane.rs

- Similar hit() functions with geometry-specific math.

### hittable.rs

- Trait Hittable: Defines shared interface for hit detection.
- Struct HitRecord: Stores hit details (point, normal, t, etc).

### hittable_list.rs

- HittableList::add(obj): Adds object to scene.
- hit(ray, t_min, t_max): Calls hit on all objects and returns the closest.

---

## 🖼️ Example Output Images

1. Sphere scene
2. Cube + plane scene with different brightness
3. Scene with all object types
4. Same scene with different camera position

Resolution: 800x600

---

## ⚙️ Installation & Usage

bash
cargo build --release
cargo run > output.ppm
convert output.ppm output.png


---

## 🧩 Adding a New Object

1. Create object.rs
2. Implement Hittable trait
3. Add hit logic
4. Add to scene in main.rs

---

## 🧪 Testing Tips

- Use low resolution first
- Add println! to debug hit logic
- Comment out heavy objects

---

## ✅ Best Practices

- Clear naming
- Use vector functions from vec3.rs
- Keep each shape in its own file
- Document each struct and function
