@_default:
    JUST_CHOOSER="sk" just --choose

# Timestamp when just action executed, use gdate from brew:coreutils on macos
ts := `gdate -u +%Y-%m-%dT%H:%M:%S.%6NZ || date -u +%Y-%m-%dT%H:%M:%S.%6NZ`

# Run a method from shared justfile
@_shared cmd *FLAGS: _sync
    just -f .cache/justfile {{cmd}} {{ts}} `pwd`

# Build for all supported targets
build:
    just _shared build-all

# Delete all build artifacts
clean:
    just _shared clean || true > /dev/null
    rm -rf .cache

# Publish binaries to GitHub release associated with current tag
publish-bins:
    just _shared publish-bins

# Publish all build artifacts
publish: publish-bins

# Sync shared justfile
_sync:
    if [[ ! -d .cache ]]; then \
        mkdir .cache; \
    fi
    if [[ ! -f .last-sync ]]; then \
        echo "1970-01-01T00:00:00.000000Z" > .last-sync; \
    fi
    if [[ `gdate --date $(cat .last-sync) +%s || date --date $(cat .last-sync) +%s` -lt `echo $(date +%s) - 86400 | bc` ]]; then \
        if [[ -d .cache/.git ]]; then \
            cd .cache; \
            git pull; \
            cd - 2>&1 > /dev/null; \
        else \
            if [[ -d .cache ]]; then rm -rf .cache; fi; \
            mkdir .cache; \
            cd .cache; \
            git clone https://github.com/sbruton/justfile .; \
            cd - 2>&1 > /dev/null; \
        fi; \
    fi

