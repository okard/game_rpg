

#version 130

uniform mat4 mvp;	//model view projection matrix
					//model -> hold model position
					//view -> camera
					//projection -> 3d world(view) to 2d screen

in vec2 my_vertex;		//input of the vertexes to render

void main()
{
	gl_Position = mvp * vec4(my_vertex, -1.0, 1.0);
}
