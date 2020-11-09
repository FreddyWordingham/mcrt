# mcrt
Monte Carlo Radiative Transfer tool

Library [![Build Status](https://travis-ci.org/FreddyWordingham/arctk.svg?branch=master)](https://travis-ci.org/FreddyWordingham/arctk)

[Documentation](https://freddywordingham.github.io/mcrt)

Input is the parameters file.
Example parameters.json5:
```json5
{
    grid: {                                                                         // Output datacube settings.
        boundary: {                                                                 // Measurement region bounding box.
            mins: [-7,-7,-7],                                                       // Domain minimum point (meters).
            maxs: [ 7, 7, 7]                                                        // Domain maximum point (meters).
        },
        res: [101,101,101]                                                          // Grid/datacube resolution.
    },
    tree: {                                                                         // Hit-scan performed using adaptive octave mesh.
        tar_tris: 5,                                                                // Target maximum number of triangles in leaf-cells.
        max_depth: 8,                                                               // Maximum splitting depth (root depth == 0).
        padding: 0.01                                                               // Multiplier to expand boxes volume when checking for contained triangles.
    },
    sett: {                                                                         // MCRT settings.
        block_size          : 1000,                                                 // Number of photons to run on a thread before asking for more.
        num_phot            : 1e6,                                                  // Total number of photons to run.
        bump_dist           : 1e-6,                                                 // Distance photons are bumped past collision boundaries (meters).
        loop_limit          : 1e6,                                                  // Maximum number of movements a photon can make before being culled (with warning).
        roulette_weight     : 1e-3,                                                 // Weight at which to start playing russian-roulette.
        roulette_barrels    : 8,                                                    // 1.0/roulette_barrels == chance of photon surviving roulette, then weight *= roulette_barrels.
        init_mat            : 'air',                                                // Initial material observed by new photons. (Detection is being stabilised).
    },
    surfs: {                                                                        // Surfaces within the system.
        plane       : [['objs/plane.obj'],     null],                               // [list of paths to base wavefront files], optional<transformation>:
        cube        : [['objs/cube.obj'],      null],                               // null == no transform
        cylinder    : [['objs/cylinder.obj'],  null],                               // rot: [x_axis_spin, y_axis_spin, z_axis_spin] (degrees)
        square      : [['objs/square.obj'],    null],                               // scale: f64, trans: [delta_x, delta_y, delta_z] (meters)
    },
    attrs: {                                                                        // Surface attributes.
        plane       : { Mirror:         {} },                                       // Completely reflective mirror.
        cube        : { Refractive:     { inside: 'fog',    outside: 'air'} },      // Material interface (refractive), inside of mesh material, outside of mesh material.
        cylinder    : { Refractive:     { inside: 'flesh', outside: 'fog'} },
        square      : { Spectrometer:   {}},                                        // Detector surface, in this case a spectrometer. (default resolution 100, 0 - 1000nm)
    },
    mats: {                                                                         // Material optical properties.
        milk: {
            Here: {                                                                 // Can be written in place:
                ref_index   : { Constant: 1.4    },                                 // Refractive index.
                scat_coeff  : { Constant: 10.0e3 },                                 // Scattering coefficient (per meter).
                abs_coeff   : { Constant: 10.0e2 },                                 // Optional absorption coefficient (per meter).
                                                                                    // Optional shifting (Raman/fluorescence) coefficient (per meter).
                asym_fact   : { Constant: 0.8    }                                  // Henyey-Greenstein asymmetry scattering factor (-1.0:1.0).
            }
        },
        air:    { There: 'mats/air.json5'   },                                      // Or re-directed to another file.
        fog:    { There: 'mats/fog.json5'   },
        flesh:  { There: 'mats/flesh.json5' }
    },
    light: {                                                                        // Emission surface.
        power       : 1.0,                                                          // Source power (watts).
        surf        : [['objs/circle.obj'], null],                                  // List of source surfaces.
        spec        : { Here: {Point: 6.3e-7}}                                      // Emission spectrum (can be redirected to another file for complex Spectrum enumerations).
    }
}
```

Output in the form of netcdf datacubes.
Example energy density of the above input:
![image info](./res/example.png)
