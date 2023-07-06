# raycasting
Prototype of a 2D raycasting application, using the [bevy engine](https://bevyengine.org). Might be used to implement a non grid based line of sight for games.

Currently, a randomly generated map will be created. Your mouse cursor is the origin of the raycast. The raycast includes the whole map.

Main sources:
- https://ncase.me/sight-and-light/
- https://www.redblobgames.com/articles/visibility/
- https://stackoverflow.com/a/565282 (for the ray segment intersection algorithm)
- https://basstabs.github.io/2d-line-of-sight/Introduction.html (not implemented, but might be interesting for optimizations)