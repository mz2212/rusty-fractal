uniform dvec2 loc;
uniform double zoom;
uniform int max_iter;
uniform uint width;
uniform uint height;
out vec3 color;

void main() {
	dvec2 old, new, pix;
	vec3 hsv;
	bool to_break;

	pix[0] = 1.5*(gl_FragCoord.x - double(width) / 2.0) / (0.5 * zoom * width) + loc.x;
	pix[1] = (gl_FragCoord.y - double(height) / 2.0) / (0.5 * zoom * height) + loc.y;

	int i;
	new[0] = 0.0;
	new[1] = 0.0;
	for(i = 0; i < max_iter; i++) {
		old = new;

		new[0] = (old[0] * old[0] - old[1] * old[1]) + pix[0];
		new[1] = 2 * (old[1] * old[0]) + pix[1];
		
		// Check if next value is > 4, if so break.
		to_break = bool((new[0] * new[0] + new[1] * new[1]) > 4.0);

		if (to_break) break;
	}

	hsv[0] = float(i % 256) / 255.0;
	hsv[1] = 1.0;
	hsv[2] = float((i < max_iter)) * 1.0;
	color = hsv2rgb(hsv);
}

