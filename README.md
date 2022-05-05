# Velvet Sky 

## The First in the Velvet Sky of Dreams Series

- [Velvet Sky](#velvet-sky)
  - [The First in the Velvet Sky of Dreams Series](#the-first-in-the-velvet-sky-of-dreams-series)
    - [Version Log](#version-log)
    - [Feature Development](#feature-development)
    - [Challenges](#challenges)
    - [Bug List](#bug-list)

### Version Log
| Version | Release   | Discussion                                                                                                                      |
| ------- | --------- | ------------------------------------------------------------------------------------------------------------------------------- |
| 0.0.0   | Mar2022   | Initial Commit; Added Meshes; Added Collisions; Added Controls;                                                                 |
| 0.1.0   | 05May2022 | Updated to Bevy 0.7.0 and Rapier 0.13 and full rewrite; Added camera following; Added forward motion (z-axis); Added colliders; |

### Feature Development
- Collision 
- Camera Tracking
  - Camera Collider
  - Camera Follower
- Buildings
- Obstacles
-  Guns
- Shields
- Scoring
- Targets
- Loading States
- Menu
- Pause
- Expand Ship Selection
- Expand Ship Engine Selection
- Achievements


### Challenges
- Loading states are having issues with over writing into new state : https://github.com/bevyengine/bevy/issues/1839
- https://bevy-cheatbook.github.io/programming/states.html

### Bug List
- Pbr::Plane functions as square of death

