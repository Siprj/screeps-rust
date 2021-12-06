# Relations

## RoomPosition.findPathTo

Calls in certain situations:

If the target is in the same room the `Room.findPath` is called

If the target is in different room this sequence of calls is invoked:

* `Room.findExitTo`
* `RoomPosition.findClosestByPath`
* `Room.findPath`

## RoomPosition.findClosestByPath

Call one heavy function `getPathFindingGrid2` to create the `CostMatrix`, and
then it uses this cost matrix in `PathFinder.search`.

Used options:

* `ignoreRoads`
* `costCallback`
* `filter` - used for filtering in `Room.find` function.

## Function `getPathFindingGrid2`

The `getPathFindingGrid2` uses following things from `opts` options:

* `ignoreCreeps`
* `ignoreDestructibleStructures`
* `ignoreRoads`

__Forbiden__ options when using new PathFinder (now default).

* `avoid`
* `ignore`

__Unused__ options:

* serialize

And it calls `makePathfindingGrid2` function. Which then uses following options:

* `ignoreCreeps`
* `ignoreDestructibleStructures`
* `ignoreRoads`

## RoomPosition.findPath

Call one heavy function `getPathFindingGrid2` to create the `CostMatrix`, and
then it uses this cost matrix in `PathFinder.search`.

Used options:

* `costCallback`
* `ignoreCreeps`
* `ignoreDestructibleStructures`
* `ignoreRoads`
* `serialize`
* `range` - it is passed to `search` function as range parameter of the target
  object.

__Forbiden__ options when using new PathFinder (now default).

* `avoid`
* `ignore`

The `roomCallback` is constructed using `getPathFindingGrid2` and `costCallback`
option.

## Game.map.findRoute

Is written using javascript based A star, because it is not too costly to compute
on room level.

Uses only `routeCallback`.

## Game.map.findExit

Uses `Game.map.findRoute` under the hood, thus it uses only `routeCallback`
option.

## Creep.moveTo

Uses `RoomPosition.findPathTo` function.
