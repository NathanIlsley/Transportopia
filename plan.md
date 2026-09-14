# The Plan

## Introduction

This will be an isometric transport simulation game. The objective will be to make money by transporting passengers around a city using trains, trams and busses.

Building transport infrastructure will improve land value around well connected stations as well as areas where traffic congestion has been reduced or eliminated. This will lead to the population in these areas increasing and therefore the demand for transport increasing as well.

Traffic congestion will be a main challenge in this game as it will reduce land value and slow busses and trams if they don't have a dedicated lane. The player will be able to tackle it by improving the road network and converting roads to be car-free, although they will have to watch out for the traffic increasing elsewhere because of this.

## Development

### Rendering

The world will need to be rendered by drawing many individual terrain tiles, road tiles and train track tiles (and the vehicles on them) to the screen. On top of this, teh buildings and other objects with height will be drawn in an order such that the buildings further up the screen appear behind the ones lower down.

In order to make rendering less expensive, the world will be divided into chunks. Each chunk will have a ground texture created by drawing the terrain and road tiles. The chunk will also have a structures texture with all the buildings on it. The vehicles will be designed such that the image doesn't exceed the tile boundaries. This means that as long as chunk has no changes to its buildings or terrain, it can be drawn to the screen by first drawing the ground texture then all the vehicles and finally the buildings on top.

### Traffic Simulation

### Building Construction by City

### Road Construction by City

### Road and Rail Construction by Player