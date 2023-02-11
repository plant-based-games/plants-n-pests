backend:
- install zod
- use zod to parse cookie
    - player id
- initial state is waiting for players
- increment player id until 4, then game auto-starts
- player 1 can start game at 2 or 3 players
- add draft endpoint
    - will have draft and play endpoints
- have a awaiting draft array & awaiting play Option<Player>
    - on draft endpoint, check draft array to validate player
    - on play endpoint, check awaiting play to validate player

$direction = left
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

You drafted 24 cards and played 6.
Now you have a deck of 18 remaining cards.
Round
Draw 6 cards and each player plays a card or passes in turn order.
Round ends once all players pass.
Repeat Round 3 times.
