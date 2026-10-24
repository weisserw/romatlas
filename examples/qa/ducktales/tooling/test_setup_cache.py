"""Cache reuse, invalidation and content integrity without game execution."""
from pathlib import Path
from tempfile import TemporaryDirectory
import setup_cache as cache

with TemporaryDirectory() as temporary:
    root = Path(temporary)
    identity = {'build': 'one', 'recipe': 'one', 'expectation': 'original-fidelity'}
    assert cache.select(root, identity) == (None, 'miss')
    with cache.candidate(root, identity) as staged:
        for name in cache.FILES:
            (staged/name).write_text('{}')
        selected = cache.publish(staged, root, identity, 'validated-natural-input-run')
    assert cache.select(root, identity) == (selected, 'hit')
    assert cache.select(root, dict(identity, build='two')) == (None, 'miss')
    assert cache.select(root, dict(identity, recipe='two')) == (None, 'miss')
    assert cache.select(root, dict(identity, expectation='modified')) == (None, 'miss')
    (selected/cache.FILES[0]).write_text('tampered')
    assert cache.select(root, identity) == (None, 'content_mismatch')
print('setup cache: identity, recipe, profile invalidation and corruption rejection passed')
