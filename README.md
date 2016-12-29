# Pong
Yet another pong clone

## Controls

* Start/pause: spacebar
* Left paddle is controlled via mouse

## To-Do

* Add a scoreboard
* Add a "PAUSED" overlay when the game is paused
* Improve collision detection
* Allow paddle to be controlled via W/S keys or up/down arrow keys
* Implement a menu screen
  * Difficulty selection
  * Allow Human or CPU selection for each paddle
* Make AI less jittery
  * Possibly redo AI to predict the ball's path,
    rather than just track its Y position

## AI

Here's how the AI currently works:

Every time you hit the ball, the AI chooses a "target": top, center, or bottom.
This is the part of the paddle that the AI tries to hit the ball with.
If the target is the top or bottom, there's additionally an offset, so that
the AI isn't trying to use the same location on the paddle every time.

While the ball is on the opposite side of the field, the AI ignores this value
and tries to get itself centered on the ball. This is to increase the frequency of
the paddle getting to the correct location fast enough. Unfortunately, this is also
causing a bit of jitter when the ball goes over the center line and the AI's target
location changes.

After hitting the ball, the AI returns the paddle to the middle of the screen.
