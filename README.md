# GivMe

A password manager built in Rust that is simple to use and safe. Save your passwords with an optional note for yourself.

## Why GivMe

- Uses 2 of the world strongest encryption algorithms.
- Dependencies < 10.
- Easy installation with `cargo`.
- Written in Rust.
- Open Source.
- Encrypted with 2 different keys. You only need to remember 1.

## GivMe Setup

```plain
$ givme
[+++] First Run Setup [+++]
Set your master key: hello123
Confirm your master key: hello123
```

## Saving Passwords

```plain
$ givme --store mypassword
Enter your Master Key: hello123
Enter your 'mypassword': thisismypassword
Any note for yourself: Please don't lost this password
Saved Successfully
```

## Retrieving Passwords

```plain
$ givme mypassword
Enter your Master Key: hello123

Here's your 'mypassword':  thisismypassword
Note: Please don't lost this password
```

### Currently Under Development
