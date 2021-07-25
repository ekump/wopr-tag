# wopr-tag
Agent based simulation engine for playing tag

## What is it
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

In the current version the players move randomly, in order, one at a time.

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

### Advanced options
By default the field of play is displayed, but this can be turned off by
including the `--show-field false` argument.

You can also set how long to wait between each turn, in milliseconds. The
default is 250ms. This allows the user to view the field of play and any
changes in who is it. To accelerate the simulation you can set this number as
low as zero by including the `--wait-between-turn 0` argument.

## Note on tests
I've included some unit tests to show that I'm not uncivilized, but I made the
conscious decision to not make them exhaustive. The simulation itself is not
tested becuse that would require complex setup beyond the scope of the project.
The public functions of the agent and other models have basic tests, but not
every logic condition is covered.

![the original wopr](http://guidetomonsters.com/img/eighties/Wop1.jpg)
