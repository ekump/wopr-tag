# wopr-tag
Agent based simulation engine for playing tag

## What is it (pun intended)
wopr-tag is a simulation engine for the game of tag. The rules are as follows:
- One and only one player is it at a time.
- When a player is it and next to another player they may tag them and now the
  other player is it.
- No tag-backs. The most recent player who was previously it may not be tagged
  again. At least one other player must be it in-between a player being it
  twice.
- A player who is it may take two actions. At the start of their turn, if they
  are able to tag another player they may do so. Then they may also take a move
  action. A player who is not it is permitted to make one move action.
- If a player gets "stuck", meaning they have nowhere to move to because they
  are at the edge of the field of play and are surrounded by other players then
  they will take no action for that turn.
- Two players cannot occupy the same space on the field.
- Players try to avoid moving closer to the it player. They are randomly
  assigned a risk tolerance. The higher the tolerance the more likely their
  random movements will move them closer to the it player.

## How to run
wopr-tag is written in Rust, which is required to compile and run. If you do
not already have Rust installed you can find directions at
https://www.rust-lang.org/tools/install.

Once you have your Rust environment setup you need to build the application.
You can simply run `cargo build --release`. This will create a binary in your
`target/release` folder called `wopr_tag`. The application has a command-line
interface with multiple options. Run `wopr_tag --help` for a full explanation.
We'll cover the basics here. In order to run the simulation you need to specify
the number of players as well as the length (x-size) and width (y-size) of the
field. The dimensions and players must be at least three. For example, to run
the simulation with 3 players on a 4x4 field you would run `wopr_tag
--num-players 3 --x-size 4 --y-size 4`.

If you are in the project directory you can compile and run the application at
the same time using the `cargo run` command.  For example: `cargo run --release
--bin wopr_tag -- --num-players 10 --x-size 500 --y-size 500`

### Advanced options
By default the field of play is displayed, but this can be turned off by
including the `--show-field false` argument.

You can also set how long to wait between each turn, in milliseconds. The
default is 250ms. This allows the user to view the field of play and any
changes in who is it. To accelerate the simulation you can set this number as
low as zero by including the `--wait-between-turn 0` argument.

You can define the number of turns the simulation will take. To set it to
100,000 you would pass the argument `--num-turns 100000`.

There is some sparse debug logging available. You can set the environment
variable `LOG_LEVEL` to `debug` for more verbose logging. For example:
`LOG_LEVEL=debug wopr_tag --num-players 3 --x-size 4 --y-size 4`. The default
is `INFO`.

## Known limitations
- The player who is it moves randomly. The realism of the simulation would
  likely be improved if the currently it player attempted to move in a
  direction towards the closet player or the most players.
- Players do not face in a specific direction. Ideally a player would have a
  certain orientation and they would only see other players in their line of
  sight and would continue moving in a forward direction with a higher
  probability.
- Players all move at the same speed and have the same endurance. This can
  easily be changed by assigning these attributes randomly during
  initialization and allowing players to move more than once per turn and
  forcing them to not move when they have no stamina.
- The statistics output is very basic.
- Only one player acts at a time, and the order in which players act is the
  same for the entire simulation. This was an intentional trade-off for the
  sake of achieving correctness and reliability first. Theoretically, the
  individual player agents can run independently on their own threads. However,
  you would need to refactor the `FieldOfPlay` cache to support a
  multi-threaded approach.
- Only one simulation runs at once. This is intentional since the viewer is so
  basic, and there aren't a lot of parameters that would meaningfully change
  the results. Assuming integration with a TUI crate it would be rather trivial
  to then run multiple simulations at once using Rayon.

## Note on tests
I've included some unit tests to show that I'm not uncivilized, but I made the
conscious decision to not make them exhaustive. The simulation itself is not
tested becuse that would require complex setup beyond the scope of the project.
The public functions of the agent and other models have basic tests, but not
every logic condition is covered.

## Namesake
![the original wopr](http://guidetomonsters.com/img/eighties/Wop1.jpg)
