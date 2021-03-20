# Yubikey GPG Watcher

If you use you yubikey to sign your git commits using GPG, then you are used to this little blinking green light that's waiting for you to click on it.

And sometimes, well, sometimes, _you forget to click on this button_ and then git gives up and you have to run your command again. That's easy if this is a commit, but if this is a complex script that waiting for CI to run remotely to merge your changes, then it's **very** easy to miss this blinking light.

This script runs on your Apple laptop and watch for git asking GPG to sign a commit. When this happens, it will pop a notification. That's all.

## Running the code

```shell
cargo run --release
```

