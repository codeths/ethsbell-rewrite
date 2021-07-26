# Contributing

Please follow these guidelines when making contributions to ETHSBell.

* Configure your IDE to use tabs to indent. It works better for everyone. You can set the size of a tab to whatever you like.
* Try to ensure that the project at least compiles for any given commit.
* We have a few tests that help ensure that things always work correctly. Please run them before pushing your code.
  * If you push a tag when the tests don't pass, the automated build will fail, because it runs tests before building anything.
* Please make sure your tags follow semver.
  * The rightmost number must be incremented only when bugs and regressions have been fixed and no additions have been made.
  * The middle number must be incremented only when features have been added and no incompatible changes have been made.
  * The leftmost number must be incremented only when incompatible changes have been made.