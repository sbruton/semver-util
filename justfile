# path to util justfiles
_justfile_dir := "../util"

@_default:
    JUST_CHOOSER="sk" just --choose

# Delete all build artifacts
clean:
    just -f {{_justfile_dir}}/justfile clean `pwd`

# Publish binaries to GitHub release associated with current tag
publish-bins:
    just -f {{_justfile_dir}}/justfile publish-bins `pwd`

# Publish all build artifacts
publish: publish-bins