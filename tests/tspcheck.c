// TSP Solution checker -- expects two file names
// on the command line argument.  The first is the
// TSP problem (number of vertices, followed by coordinates).
// The second is a TSP solution, which is the number of vertices,
// followed by the index number of the vertex.
//
// Checks to see if all vertices are visited, and then prints
// the length of the tour.

#include <stdio.h>
#include <math.h>
#include <stdlib.h>

typedef struct {
  float x, y;
} vertex;

#define MAXVERTEX 2000
vertex vertices[MAXVERTEX];
int tour[MAXVERTEX];
int num_vertices;

float distance(int v1, int v2)
{
  float dx = vertices[v1].x - vertices[v2].x;
  float dy = vertices[v1].y - vertices[v2].y;
  float d = sqrt(dx*dx + dy*dy);
  printf("%f %f -> %f %f = %f\n", vertices[v1].x, vertices[v1].y, vertices[v2].x, vertices[v2].y, d);

  return d;
}


int main(int argc, char *argv[])
{
  FILE *fp = fopen(argv[1], "r");
  fscanf(fp, "%d", &num_vertices);
  for (int i = 0; i < num_vertices; ++i)
    fscanf(fp, "%f %f", &vertices[i].x, &vertices[i].y);
  fclose(fp);

  int num_soln;
  fp = fopen(argv[2], "r");
  fscanf(fp, "%d", &num_soln);
  if (num_soln != num_vertices)
    {
      printf("Wrong number of vertices in the solution\n");
      exit(1);
    }
  
  // Mark vertices as unvisited
  for (int i = 0; i < num_vertices; ++i)
    tour[i] = -1;
  
  for (int i = 0; i < num_vertices; ++i)
    {
      fscanf(fp, "%d", &tour[i]);
    }

  // Check to see if everythign got marked
  for (int i = 0; i < num_vertices; ++i)
    {
      if (tour[i] == -1)
	{
	  printf("Tour did not visit vertex %d\n", i);
	  exit(1);
	}
    }

  // Now compute the tour length
  float total = 0;
  for (int i = 0; i < num_vertices; ++i)
    {
      total = total + distance(tour[i], tour[(i + 1) % num_vertices]);
    }
  printf("Total tour: %f\n", total);
}
