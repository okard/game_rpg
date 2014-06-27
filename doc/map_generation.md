# Map Generation


Notice: The Legend of Zelda: Link's Awakening DX 8x8 Chunks -> 128x128 Tiles map

## Info

Overview of Layers:

* Ground Tiles
	- Basic Layer 0
* Over Layer
	- First Layer not passable static objects
	- Additional 'Roof' Layer for Buildings
* Moveable area
	- Information Layer generate at runtime?
* Entity Layer
	- Holding Entities (dynamic stuff)
	- Interact with input
	- Interact with other entities
* Temp Layer
	- Editor 
	- Placement of Stuff
	
Map will be splitted in chunks

## Algorithm

1. Generate Tilemap in Size
	- 4000x4000 tiles with 2 byte ids per tile
		-> 30 MB 
	- Compression?

2. Drop Basic Regions
	- Big Landscape
		- Dessert
		- Grassland
		- Swamp
		- Mountains
	- Small Landscape
		- Lake
	- Villages
	- Buildings
	
	- Noise on tiles
		- Check distance between points
		
3. Rework Borders?
	- Not passable cliffs
	- Ocean
	- Looping
	
4. Connections
	- Ways
	- Pathes
	- River
	
5. Entity Placement
	- Map Constraints
		- Must contain specific stuff
		
		
## Logical

* Shared
	* Terrain Regions + Ids
* Map
	* Layer
	* Entity Status
	* Spawn Rates
	* References to other maps
		- Sub Maps?
		- Dungeons
		- Inside of buildings
		
## Rendering

* Animated tiles
* Light/Normal Maps
* Tilesets
* Entities


