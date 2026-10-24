"""Historical paired-navigation prototype, retained for diagnostics only.

Default gameplay QA uses scripts/native_gameplay.py without the reference.
Ordinary-gameplay navigation only; acceptance assertions belong to scenarios.

Batch only known-safe ordinary gameplay spans. Stop before dialogue/vertical/death
boundaries and use their dedicated owner handlers. A failed waypoint invalidates
the branch; restore its bookmark before trying a smaller span.
"""
from g_routes import finish_source_frame, health
from j_climb import world_facts


def advance_waypoint(pair, ticks, label, native_session=None):
    if not isinstance(ticks, int) or isinstance(ticks, bool) or not 1 <= ticks <= 8:
        raise ValueError('navigation batches must be 1..8 ticks')
    before = pair.game()
    assert before['mode'] == 'gameplay' and before['calls']['vertical'] is None
    if native_session is None:
        pair.advance(ticks, 'native')
    else:
        native_session.advance(ticks)
    game = pair.game()
    assert game['mode'] == 'gameplay' and game['calls']['vertical'] is None, 'restore and use boundary-specific owner handling'
    assert game['player']['selector'] not in [13, 15], 'restore before death/dialogue boundary'
    pair.advance(ticks, 'reference')
    finish_source_frame(pair, pair.evidence, game, label)
    pair.compare_player(label, game)
    health(pair, game)
    return game


def validate_waypoint(pair, game, label):
    """Validate a retained setup at a deliberate waypoint, never grant QA acceptance."""
    pair.compare_player(label, game)
    world_facts(pair, game, label)
    health(pair, game)
    assert pair.memory('StatusDigitCodes', 6) == game['status']['reward']
