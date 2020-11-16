# mcrt
Monte Carlo Radiative Transfer tool

<!-- [![ANTLER documentation](https://docs.rs/mcrt/badge.svg)](https://docs.rs/mcrt) -->
[![ANTLER crate](https://img.shields.io/crates/v/arctk.svg)](https://crates.io/crates/mcrt)
[![ANTLER documentation](https://docs.rs/mcrt/badge.svg)](https://freddywordingham.github.io/mcrt/)
[![minimum rustc 1.47](https://img.shields.io/badge/rustc-1.47+-red.svg)](https://www.rust-lang.org/)
[![Build Status](https://travis-ci.org/FreddyWordingham/arctk.svg?branch=master)](https://travis-ci.org/FreddyWordingham/arctk)

Input is the parameters file.
Example parameters.json5:
```json5
{
    tree    : { There   : 'tree/deep_5_8.json5' },
    grid    : { There   : 'grid/20m_64.json5' },
    sett    : { Here    : {
        num_phot            : 1e6,
        block_size          : 1e2,
        bump_dist           : 1e-6,
        loop_limit          : 1e6,
        min_weight          : 5e-2,
        roulette_barrels    : 8,
        init_mat            : 'air',
    }},
    engine  : "Basic",
    light   : { Here    : {                                                     // Use these light settings.
        power       : 1.0,                                                      // Power [watts].
        emit        : { Surface: [['objs/light.obj'], null] },                  // Emit from a mesh surface - no transformations.
        spec        : { Here: {Point: 6.3e-7}}                                  // Use this laser spectrum (630nm).
    } },
    mats    : { Here    : {                                                     // Use this material map.
        air : { There   : 'mats/air.json5' },                                   // Air properties there.
        fog : { There   : 'mats/fog.json5' },                                   // Fog properties there.
    }},
    attrs   : { Here    : {                                                     // Use these attributes.
        mirr    : { Mirror: 0.99 },                                             // 99% reflective mirror.
        fog     : { Refractive: ['air', 'fog'] },                               // Interface between air and fog.
    }},
    surfs   : { Here    : {                                                     // Use these surfaces.
            centrepiece : {
                mesh    : [["objs/cube.obj"], {scale: 1.1} ],                   // Use this surface - scale it up 10%.
                attr    : 'fog'                                                 // Make it an interface.
            },
            mirror      : {
                mesh    : [["objs/mirrors.obj"], null],                         // Use this surface - no transformations.
                attr    : 'mirr'                                                // Make it reflective.
            }
    } },
}
```

Output in the form of netcdf datacubes.
Example energy density of the above input:
![image info](./res/example.png)
