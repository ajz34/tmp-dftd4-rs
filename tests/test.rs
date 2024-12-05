use rest_dftd4::prelude::*;

mod test {
    use super::*;

    #[test]
    fn test_get_properties() {
        #[rustfmt::skip]
        let coords = [
            [ 0.00000000000000,  0.00000000000000, -0.73578586109551],
            [ 1.44183152868459,  0.00000000000000,  0.36789293054775],
            [-1.44183152868459,  0.00000000000000,  0.36789293054775],
        ];
        let coords = coords.iter().flatten().map(|&x| x).collect::<Vec<f64>>(); // coordinates needs to be flatten
        let natoms = 3;
        let charges = [8, 1, 1];
        let structure = DFTD4Structure::new(natoms, &charges, &coords, None, None, None);
        let model = DFTD4Model::new(&structure);
        let properties = get_properties(&structure, &model);
        let alpha = properties.3;
        assert!((alpha[0] - 6.74893641).abs() < 1e-6);
        assert!((alpha[1] - 1.33914933).abs() < 1e-6);
        assert!((alpha[2] - 1.33914933).abs() < 1e-6);
        /*
            >>> from dftd4.interface import DispersionModel
            >>> import numpy as np
            >>> disp = DispersionModel(
            ...     positions=np.array([  # Coordinates in Bohr
            ...         [+0.00000000000000, +0.00000000000000, -0.73578586109551],
            ...         [+1.44183152868459, +0.00000000000000, +0.36789293054775],
            ...         [-1.44183152868459, +0.00000000000000, +0.36789293054775],
            ...     ]),
            ...     numbers = np.array([8, 1, 1]),
            ... )
            >>> disp.get_properties()["polarizibilities"]
            array([6.74893641, 1.33914933, 1.33914933])
        */
    }

    #[test]
    fn test_get_dispersion() {
        #[rustfmt::skip]
        let coords = [
            [ 2.79274810283778,  3.82998228828316, -2.79287054959216],
            [-1.43447454186833,  0.43418729987882,  5.53854345129809],
            [-3.26268343665218, -2.50644032426151, -1.56631149351046],
            [ 2.14548759959147, -0.88798018953965, -2.24592534506187],
            [-4.30233097423181, -3.93631518670031, -0.48930754109119],
            [ 0.06107643564880, -3.82467931731366, -2.22333344469482],
            [ 0.41168550401858,  0.58105573172764,  5.56854609916143],
            [ 4.41363836635653,  3.92515871809283,  2.57961724984000],
            [ 1.33707758998700,  1.40194471661647,  1.97530004949523],
            [ 3.08342709834868,  1.72520024666801, -4.42666116106828],
            [-3.02346932078505,  0.04438199934191, -0.27636197425010],
            [ 1.11508390868455, -0.97617412809198,  6.25462847718180],
            [ 0.61938955433011,  2.17903547389232, -6.21279842416963],
            [-2.67491681346835,  3.00175899761859,  1.05038813614845],
            [-4.13181080289514, -2.34226739863660, -3.44356159392859],
            [ 2.85007173009739, -2.64884892757600,  0.71010806424206],
        ];
        let coords = coords.iter().flatten().map(|&x| x).collect::<Vec<f64>>(); // coordinates needs to be flatten
        let natoms = 16;
        let charges = [1, 1, 6, 5, 1, 15, 8, 17, 13, 15, 5, 1, 9, 15, 1, 15];
        let structure = DFTD4Structure::new(natoms, &charges, &coords, None, None, None);
        let model = DFTD4Model::new(&structure);
        let params = DFTD4Param::load_rational_damping("SCAN", false);
        let dispersion = get_dispersion(&structure, &model, &params, false, false);
        let energy = dispersion.0;
        assert!((energy - -0.005328888532435093).abs() < 1e-6);
        /*
            >>> from dftd4.interface import DampingParam, DispersionModel
            >>> import numpy as np
            >>> numbers = np.array([1, 1, 6, 5, 1, 15, 8, 17, 13, 15, 5, 1, 9, 15, 1, 15])
            >>> positions = np.array([  # Coordinates in Bohr
            ...     [+2.79274810283778, +3.82998228828316, -2.79287054959216],
            ...     [-1.43447454186833, +0.43418729987882, +5.53854345129809],
            ...     [-3.26268343665218, -2.50644032426151, -1.56631149351046],
            ...     [+2.14548759959147, -0.88798018953965, -2.24592534506187],
            ...     [-4.30233097423181, -3.93631518670031, -0.48930754109119],
            ...     [+0.06107643564880, -3.82467931731366, -2.22333344469482],
            ...     [+0.41168550401858, +0.58105573172764, +5.56854609916143],
            ...     [+4.41363836635653, +3.92515871809283, +2.57961724984000],
            ...     [+1.33707758998700, +1.40194471661647, +1.97530004949523],
            ...     [+3.08342709834868, +1.72520024666801, -4.42666116106828],
            ...     [-3.02346932078505, +0.04438199934191, -0.27636197425010],
            ...     [+1.11508390868455, -0.97617412809198, +6.25462847718180],
            ...     [+0.61938955433011, +2.17903547389232, -6.21279842416963],
            ...     [-2.67491681346835, +3.00175899761859, +1.05038813614845],
            ...     [-4.13181080289514, -2.34226739863660, -3.44356159392859],
            ...     [+2.85007173009739, -2.64884892757600, +0.71010806424206],
            ... ])
            >>> model = DispersionModel(numbers, positions)
            >>> res = model.get_dispersion(DampingParam(method="scan"), grad=False)
            >>> res.get("energy")  # Results in atomic units
            -0.005328888532435093
        */
    }

    #[test]
    fn test_pairwise_dispersion() {
        #[rustfmt::skip]
        let coords = [
            [-2.983345508575, -0.088082052767,  0.000000000000],
            [ 2.983345508575,  0.088082052767,  0.000000000000],
            [-4.079203605652,  0.257751166821,  1.529856562614],
            [-1.605268001556,  1.243804812431,  0.000000000000],
            [-4.079203605652,  0.257751166821, -1.529856562614],
            [ 4.079203605652, -0.257751166821, -1.529856562614],
            [ 1.605268001556, -1.243804812431,  0.000000000000],
            [ 4.079203605652, -0.257751166821,  1.529856562614],
        ];
        let coords = coords.iter().flatten().map(|&x| x).collect::<Vec<f64>>(); // coordinates needs to be flatten
        let natoms = 8;
        let charges = [7, 7, 1, 1, 1, 1, 1, 1];
        let structure = DFTD4Structure::new(natoms, &charges, &coords, None, None, None);
        let model = DFTD4Model::new(&structure);
        let params = DFTD4Param::load_rational_damping("TPSS", false);
        let pairwise = get_pairwise_dispersion(&structure, &model, &params);
        assert!((pairwise.0.iter().sum::<f64>() - -0.0023605238432524104).abs() < 1e-6);
        assert!((pairwise.1.iter().sum::<f64>() - 8.794562567135391e-08).abs() < 1e-12);
    }
}
