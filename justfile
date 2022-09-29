# path to util justfiles
_justfile_dir := "../util"

@_default:
    JUST_CHOOSER="sk" just --choose

# Publish binaries to GitHub release associated with current tag
publish-artifacts:
    just --file {{_justfile_dir}}/justfile _publish-artifacts