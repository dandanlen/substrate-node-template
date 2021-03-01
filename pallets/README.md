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
    - Bonus: the slice capacity can be configured as a runtime paramter.
2. Any network participant should be able to insert a slice into the toaster.
    - You can insert as many slices as you want, one slice at a time.
    - Bonus: configurable low-carb mode sets a limit of one slice per participant.
3. If the toaster is full, bad luck, return an error.
4. You can't take your slice back once it's inserted (danger of electrocution).
5. The last person to insert a slice shall start the toaster.
6. It takes four blocks to toast bread. 
7. Once the bread is toasted, the toaster is emptied, participants get their toast. 

> The following are explicitly out of scope: 
> - Staking and consensus
> - Cryptography
