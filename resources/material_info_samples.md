How to use it OpenGL Multiply the shininess by 128!

mat[0] = ambr; mat[1] = ambg; mat[2] = ambb; mat[3] = 1.0; glMaterialfv(GL_FRONT, GL_AMBIENT, mat); mat[0] = difr;
mat[1] = difg; mat[2] = difb; glMaterialfv(GL_FRONT, GL_DIFFUSE, mat); mat[0] = specr; mat[1] = specg; mat[2] = specb;
glMaterialfv(GL_FRONT, GL_SPECULAR, mat); glMaterialf(GL_FRONT, GL_SHININESS, shine * 128.0); VRML97 Compute
ambientIntensity as (0.212671*ambr + 0.715160*ambg + 0.072169*ambb)/(0.212671*difr + 0.715160*difg + 0.072169*difb)

Material { ambientIntensity amb diffuseColor difr digg difb specularColor specr specg specb shininess shine }