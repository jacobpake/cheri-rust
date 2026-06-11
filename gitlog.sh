git log origin/main..origin/beta \
    --cherry-pick \
    --right-only \
    --no-merges \
    --reverse \
    --format="%H" | xargs git cherry-pick
