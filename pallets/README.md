# Toaster

[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/dandanlen/substrate-node-template)

Welcome to the toaster challenge!

## Motivation

We will build a decentralised toaster pallet because, clearly, centralised toasters are unfair and inefficient. 

Your old-fashioned centralised toaster at home likely has space for four slices of bread. But you are no glutton, and moreover you 
you want to keep your carb intake under control, you only want one slice! Your neighbour may not have a toaster at all, but 
nonetheless from time to time may have a craving for that warm crispy comfort that only a fresh
slice of toast can provide. You could offer your spare toasting capacity to your neighbour, but you don't
trust *anyone* enough to allow unfettered access to your precious kitchen gadget. 

Decentralised trust-minimised toasters are the obvious solution to this conundrum. 

## Requirements

1. Our toaster will have a capacity of four slices. 
2. Any network participant should be able to insert a single slice into the toaster, but no more.
3. If the toaster is full, bad luck, return an error.
4. You can't take your slice back once it's inserted (danger of electrocution).
5. As soon as the toaster is full anyone with a slice in it can turn it on to toast the slices.
6. Toasting the slices should:
    - Empty the toaster.
    - Emit an event or multiple events signifying the toasted slices. 

> The following are explicitly out of scope: 
> - Staking and consensus
> - Cryptography
> - Weights
> - Toastenomics

## Bonus points / Variants

- Multi-slice:
    - Anyone can toast multiple slices
- Fork (can be hard or soft):
    - An account can remove its slice from a toaster (only if not switched on!)
- Delay: 
    - Toasting takes N blocks, where is a param chosen by the account starting the toaster.
    - Or each account votes on a duration when inserting their slice, duration is the median.
- Configuration:
    - configurable toaster capacity
    - configurable limit on slices per user
    - Configurable max toasting duration
