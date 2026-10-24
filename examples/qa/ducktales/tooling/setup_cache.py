"""Identity- and content-checked paired natural-play setup cache.

Entries are immutable directories, published by atomic rename only after their
native replay and reference snapshot pass the caller's semantic validation.
"""
from contextlib import contextmanager
import hashlib
import json
from pathlib import Path
import shutil
import tempfile

FILES = ('climb-entry-checkpoint.json', 'climb-entry-reference-snapshot.json',
         'climb-entry-preconditions.json')


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def cache_key(identity):
    return hashlib.sha256(json.dumps(identity, sort_keys=True, separators=(',', ':')).encode()).hexdigest()


def select(root, identity):
    directory = Path(root)/cache_key(identity)
    if not directory.exists():
        return None, 'miss'
    try:
        manifest = json.loads((directory/'cache.json').read_text())
        if manifest['identity'] != identity:
            return None, 'identity_mismatch'
        if set(manifest['files']) != set(FILES):
            return None, 'incomplete'
        if any(not (directory/name).is_file() or digest(directory/name) != expected
               for name, expected in manifest['files'].items()):
            return None, 'content_mismatch'
    except (OSError, ValueError, KeyError, TypeError):
        return None, 'invalid_manifest'
    return directory, 'hit'


@contextmanager
def candidate(root, identity):
    """Stage files; callers validate them before explicitly publishing."""
    root = Path(root)
    root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='.candidate-', dir=root) as temporary:
        yield Path(temporary)


def publish(staged, root, identity, origin):
    staged = Path(staged)
    manifest = dict(version=1, identity=identity, origin=str(origin),
                    files={name: digest(staged/name) for name in FILES})
    (staged/'cache.json').write_text(json.dumps(manifest, indent=2)+'\n')
    destination = Path(root)/cache_key(identity)
    if destination.exists():
        existing, reason = select(root, identity)
        if existing is None:
            # Preserve the bad entry as evidence; never silently overwrite it.
            raise ValueError(f'existing cache entry invalid: {reason}')
        return existing
    staged.rename(destination)
    # Recreate the disposable staging path for TemporaryDirectory cleanup.
    staged.mkdir()
    return destination


def copy_entry(source, target):
    for name in FILES:
        shutil.copyfile(Path(source)/name, Path(target)/name)
