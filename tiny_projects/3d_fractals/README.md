Q5)
Description: We quite liked your fractal example in your repository. Currently, you are rendering
2D fractals. Please, either: take this as an opportunity to extend this model to 3D and choose a
3D curve/fractal of your choice to present, OR come prepared to discuss such a project.

# Ray Marching

For each ray projects from the camera

1. Compute the signed distance function (SDF) for the objects
1. Find the smallest distance and increment by that distance
1. If the distance is really small or negative, that means that you have collided with an object and can stop
