# timestamp when just action executed, use gdate from brew:coreutils on macos
ts := `gdate -u +%Y-%m-%dT%H:%M:%S.%6NZ || date -u +%Y-%m-%dT%H:%M:%S.%6NZ`

# sync shared justfile
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

@_default:
    JUST_CHOOSER="sk" just --choose

# Delete all build artifacts
clean: _sync
    just -f .cache/justfile clean `pwd` || true > /dev/null
    rm -rf .cache

# Publish binaries to GitHub release associated with current tag
publish-bins: _sync
    just -f .cache/justfile publish-bins `pwd`

# Publish all build artifacts
publish: publish-bins