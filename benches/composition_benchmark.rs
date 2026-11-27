//! Element-count extraction from parsed chemical formulas.

use core::{hint::black_box, time::Duration};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use molecular_formulas::{ChemicalFormula, MolecularFormula, errors::CountError, prelude::Element};

fn count_separately(formula: &ChemicalFormula) -> Result<Vec<u32>, CountError> {
    (1..=u8::from(Element::Og))
        .map(|number| formula.count_of_element(Element::try_from(number).unwrap()))
        .collect()
}

fn benchmark_composition_extraction(criterion: &mut Criterion) {
    use Element::{C, Cu, H, O, S};

    let fixtures: &[(&str, &[(Element, u32)])] = &[
        ("C6H12O6", &[(C, 6), (H, 12), (O, 6)]),
        ("CuSO4.5H2O", &[(Cu, 1), (S, 1), (O, 9), (H, 10)]),
        ("[13C]CH3D", &[(C, 2), (H, 4)]),
        ("(CH2)65535", &[(C, 65_535), (H, 131_070)]),
    ];
    let mut group = criterion.benchmark_group("composition_extraction");
    group.warm_up_time(Duration::from_millis(150));
    group.measurement_time(Duration::from_millis(400));
    group.sample_size(50);
    for &(input, totals) in fixtures {
        let formula: ChemicalFormula = input.parse().unwrap();
        let mut expected = vec![0; 118];
        for &(element, count) in totals {
            expected[usize::from(element) - 1] = count;
        }
        assert_eq!(formula.element_count_vector::<u32>().unwrap(), expected, "{input}");
        assert_eq!(count_separately(&formula).unwrap(), expected, "{input}");

        group.bench_function(BenchmarkId::new("per_element", input), |bencher| {
            bencher.iter(|| count_separately(black_box(&formula)));
        });
        group.bench_function(BenchmarkId::new("one_pass", input), |bencher| {
            bencher.iter(|| black_box(&formula).element_count_vector::<u32>());
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_composition_extraction);
criterion_main!(benches);
