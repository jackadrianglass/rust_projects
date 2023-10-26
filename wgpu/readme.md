# Learning WGPU

In short, let's learn some graphics programming that isn't opengl since it appears
that rust isn't well suited to that particular architecture and wgpu is becoming
a very prevalent technology.

# Where to start

[Wgpu tutorial](https://sorth.github.io/lear-wgpu)

After doing that, start implementing gravity for an object and a system to integrate
gravity over time on the positions of the points to make them fall.

From there, work on getting one point fixed, and the other movable by some mass. Impl
hooks law and then you have a spring simulation.
