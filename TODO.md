- more auto run functons that can have lists of colors and or texts to cycle through, that can stop at a certain point, that can take in a LoadingBar instead of creating a new one, etc
- make proper documentation on docs.rs and in the README.md
- figure out how to manage auto run functions from outside the auto run function (because of how threading works)
- change panics to Result type so that they can be handled by the user
- auto run change type function that takes a vector of hash maps where the key is a percent or index into a loading bar and the value is a color or text it will accept an emum wether it is a percent or an index
- have change at type functions that will change something text or color at a certain index or percent of the loading bar
- have default auto run functions that can be used will have extra parameters that can be used to set starting colors and texts, etc.
- autorun function that stop at a certain percent or index, and retrun the textloadingbar or loadingbar that was used
- have fields that can set the ▉ to a different character and to set the last character to a different character ie having something like `[======>   ]`