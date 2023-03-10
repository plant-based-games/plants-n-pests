backend:
- State:
    - internal vs external
    - internal has all public and private info for all players (decks+hands of each player).
    - external: has all public info, but only has private info of client player (deck+hand).  Only public info for other players is provided (hand size & deck size).
    - also, external includes the client player number (1,2,3 or 4)
    - have externalize function to transform internal state to external state
- login should just return an external state
- have an awaiting draft array & awaiting play Option<Player>
    - on draft endpoint, check draft array to validate player
    - on play endpoint, check awaiting play to validate player

frontend/client:
- Game state handler
  - Scenes
    - Main menu
      - Start game
      - Profile editor (username, etc.)
      - Deck management?
    - Game lobby
      - shows you who you're playing with
    - Draft Scene
      - shows you the pile you can draft from
    - Game Scene
    - Game end scene
      - shows you your rewards, the results, etc
- UI
  - card mesh and assets
  - board meshes and assets
- Menu
  - menu buttons, font rendering
- Game State
  - menu, loading, in game, end game? 
- Networking plugin
  - exchange game data with the server
  - switching to naia for the http client because reqwest::blocking can't compile to wasm
  - bevy's runtime conflicts with reqwest's async runtime (tokio) but it should be possible to make them play nice
- Audio?


Game flow
---------

### Draft phase ###

- $direction = left
- Draft
- Each player dealt 12 cards
    - Draft cycle
        - Draft 1 cards, pass to $direction; repeat 4 times
        - Now you have a hand of 4 cards
        - Play 1 card from hand of 4 in turn order
        - Arrange the 3 remaining drafted cards in any order and add each card into deck as you scroll through deck top-to-bottom 1-way.
    - Repeat Draft cycle 3 times
- toggle $direction
- Repeat Draft 2 times


### Round phase ###

- You drafted 24 cards and played 6.
- Now you have a deck of 18 remaining cards.
- Round
- Draw 6 cards and each player, in turn order, takes a turn:
- A turn can be 1 of 4 actions:
    - play a card
    - harvest+sell 1 previously planted plant
    - fulfill a contract
    - pass your turn
- Round ends once all players pass.
- Repeat Round 3 times.


Ideas
-----

Goal: grow plants to sell for money (victory points) at the market or to satisfy contracts.  Player with most money at end of game wins.


### Garden ###

5 x 5 grid of empty plots.  This is a shared space.  Each cell in the grid can have 1 terrain and 1 plant.


### Card types ###

- contracts
- terrain
- plants

- contract: requires you to harvest a fixed quantity of plant(s).  Rewards you with a fixed price if requirement met.  Otherwise, pay a monetary penalty.
    - requirement
    - reward
    - penalty
- terrain: play into an empty grid cell to claim that cell and meet plant planting requirements.
- plant: play into one of your terrains if that terrain & its surrounding terrains meets the plant's requirements.  Each plant type increases the yield of certain other adjacent plants, and decreases the yield of certain other adjacent plants.  On any subsequent turn, you can harvest and sell the plant you previously planted.  Certain plants have yield benefits if adjacent to certain terrain.   Certain plants have yield penalties if adjacent to certain terrain.


### Market price ###

- Each plant type starts at a random market price.
- The market price for a given plant goes up each round no one sells any of that plant type.
- The market price for a given plant goes down each round someone sells any of that plant type.


### Bootstrap ###

- index.html loads html/js/css/wasm
- call login to get cookie
- All players can see number of players joined so far
- once all players have called login (3 or 4 players, configured in .env player_count env var), start game
- download the card-library GET /card-library


### Communication ###

- When waiting for other players, frontend calls GET pollState every 1 second and updates view with new GameState.
- When current player's turn, once the player makes their choice, frontend calls appropriate endpoint to update game state
    - draft
    - play
    - play-contract
