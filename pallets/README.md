# Toaster

Welcome to the toaster challenge!

## Motivation

We will build a decentralised toaster because, clearly, centralised toasters are unfair and inefficient. 

Your toaster likely has space for four slices of bread. But you are no glutton, and moreover you don't
want to go overboard on your carbs, you only want one slice! Your neighbour may not have a toaster at all, but 
nonetheless from time to time may have a craving for that warm crispy goodness comfort that only a fresh
slice of toast can provide. You could offer your spare toasting capacity to your neighbour, but you don't
trust her enough to allow unfettered access to your precious kitchen gadget. 

Decentralised trust-minimised toasters are the obvious solution to this conundrum. 

## Requirements

1. Our toaster will have a capacity of four slices. 
2. Any network participant should be able to insert a slice into the toaster.
    - You can only insert a single slice.
3. If the toaster is full, bad luck, return an error.
4. You can't take your slice back once it's inserted (danger of electrocution).
5. If the toaster is full anyone with a slice in it can turn it on to toast the slices.
6. Toasting the slices should:
    - Empty the toaster.
    - Return an event or multiple events signifying the toasted slices. 

## Bonus points

- Multi-slice:
    - Anyone can toast multiple slices
- Delay: 
    - Toast takes N blocks 
    - N might be configurable or random within a given range
- Configuration:
    - configurable toaster capacity
    - configurable limit on slices per user
    - Configurable toasting duration

> The following are explicitly out of scope: 
> - Staking and consensus
> - Cryptography
> - Weights
