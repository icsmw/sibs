use crate::{
    elements::*,
    error::LinkedErr,
    inf::{operator::E, tests::*, Configuration},
    read_string,
    reader::{Dissect, Reader, Sources},
};
use proptest::prelude::*;

mod ppm {

    use crate::{
        elements::{Accessor, Call, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    fn reading_call(call: Call) {
        get_rt().block_on(async {
            let origin = format!("@test {{\nsome_initial_func(){call};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    let task = src
                        .report_err_if(Task::dissect(reader))?
                        .expect("Task read");
                    assert_eq!(format!("{task};"), origin);
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        })
    }

    fn reading_accessor(acs: Accessor) {
        get_rt().block_on(async {
            let origin = format!("@test {{\nsome_initial_func(){acs};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    let task = src
                        .report_err_if(Task::dissect(reader))?
                        .expect("Task read");
                    assert_eq!(format!("{task};"), origin);
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 5000,
            ..ProptestConfig::with_cases(10)
        })]
        #[test]
        fn test_run_calls(
            args in any_with::<Call>(0)
        ) {
            reading_call(args.clone());
        }
        #[test]
        fn test_run_accessors(
            args in any_with::<Accessor>(0)
        ) {
            reading_accessor(args.clone());
        }
    }
}

impl Arbitrary for Metadata {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![Just(true), Just(false),]
            .prop_map(|tolerance| Metadata {
                comments: Vec::new(),
                meta: Vec::new(),
                ppm: None,
                tolerance,
                inverting: false,
                token: 0,
            })
            .boxed()
    }
}

fn generate(targets: &[ElementRef], deep: usize) -> Vec<BoxedStrategy<Element>> {
    let mut collected = Vec::new();
    if targets.contains(&ElementRef::Range) {
        collected.push(
            Range::arbitrary()
                .prop_map(|el| Element::Range(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Call) {
        collected.push(
            Call::arbitrary()
                .prop_map(|el| Element::Call(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Accessor) {
        collected.push(
            Accessor::arbitrary()
                .prop_map(|el| Element::Accessor(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Combination) {
        collected.push(
            Combination::arbitrary()
                .prop_map(|el| Element::Combination(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Conclusion) {
        collected.push(
            Conclusion::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Conclusion(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Closure) {
        collected.push(
            Closure::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Closure(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Loop) {
        collected.push(
            Loop::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Loop(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::While) {
        collected.push(
            While::arbitrary_with(deep + 1)
                .prop_map(|el| Element::While(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Incrementer) {
        collected.push(
            Incrementer::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Incrementer(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Error) {
        collected.push(
            Error::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Error(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Return) {
        collected.push(
            Return::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Return(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Compute) {
        collected.push(
            Compute::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Compute(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Breaker) {
        collected.push(
            Breaker::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Breaker(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Join) {
        collected.push(
            Join::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Join(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Subsequence) {
        collected.push(
            Subsequence::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Subsequence(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Condition) {
        collected.push(
            Condition::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Condition(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Integer) {
        collected.push(
            Integer::arbitrary()
                .prop_map(|el| Element::Integer(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Boolean) {
        collected.push(
            Boolean::arbitrary()
                .prop_map(|el| Element::Boolean(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Block) {
        collected.push(
            Block::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Block(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Command) {
        collected.push(
            (
                Command::arbitrary_with(deep + 1),
                Metadata::arbitrary_with(()),
            )
                .prop_map(|(el, md)| Element::Command(el, md))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Comparing) {
        collected.push(
            Comparing::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Comparing(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Component) {
        collected.push(
            Component::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Component(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Each) {
        collected.push(
            Each::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Each(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::First) {
        collected.push(
            First::arbitrary_with(deep + 1)
                .prop_map(|el| Element::First(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::For) {
        collected.push(
            For::arbitrary_with(deep + 1)
                .prop_map(|el| Element::For(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Function) {
        collected.push(
            (
                Function::arbitrary_with(deep + 1),
                Metadata::arbitrary_with(()),
            )
                .prop_map(|(el, md)| Element::Function(el, md))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::If) {
        collected.push(
            If::arbitrary_with(deep + 1)
                .prop_map(|el| Element::If(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::IfThread) {
        collected.push(
            IfThread::arbitrary_with((0, deep + 1))
                .prop_map(|el| Element::IfThread(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::IfCondition) {
        collected.push(
            IfCondition::arbitrary_with(deep + 1)
                .prop_map(|el| Element::IfCondition(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::IfSubsequence) {
        collected.push(
            IfSubsequence::arbitrary_with(deep + 1)
                .prop_map(|el| Element::IfSubsequence(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Meta) {
        collected.push(Meta::arbitrary().prop_map(Element::Meta).boxed());
    }
    if targets.contains(&ElementRef::Optional) {
        collected.push(
            Optional::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Optional(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Gatekeeper) {
        collected.push(
            Gatekeeper::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Gatekeeper(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::PatternString) {
        collected.push(
            PatternString::arbitrary_with(deep + 1)
                .prop_map(|el| Element::PatternString(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Reference) {
        collected.push(
            Reference::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Reference(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Task) {
        collected.push(
            Task::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Task(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Values) {
        collected.push(
            Values::arbitrary_with(deep + 1)
                .prop_map(|el| Element::Values(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::VariableAssignation) {
        collected.push(
            VariableAssignation::arbitrary_with(deep + 1)
                .prop_map(|el| Element::VariableAssignation(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::VariableName) {
        collected.push(
            VariableName::arbitrary()
                .prop_map(|el| Element::VariableName(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::VariableType) {
        collected.push(
            VariableType::arbitrary()
                .prop_map(|el| Element::VariableType(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::VariableDeclaration) {
        collected.push(
            VariableDeclaration::arbitrary_with(deep)
                .prop_map(|el| Element::VariableDeclaration(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::VariableVariants) {
        collected.push(
            VariableVariants::arbitrary()
                .prop_map(|el| Element::VariableVariants(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::SimpleString) {
        collected.push(
            SimpleString::arbitrary()
                .prop_map(|el| Element::SimpleString(el, Metadata::default()))
                .boxed(),
        );
    }
    if targets.contains(&ElementRef::Comment) {
        collected.push(Comment::arbitrary().prop_map(Element::Comment).boxed());
    }
    collected
}

impl Arbitrary for Element {
    type Parameters = (Vec<ElementRef>, usize);
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((targets, deep): Self::Parameters) -> Self::Strategy {
        prop::strategy::Union::new(generate(&targets, deep)).boxed()
    }
}

fn reading(el: Element) {
    get_rt().block_on(async {
        let origin = format!("{el};");
        read_string!(
            &Configuration::logs(false),
            &origin,
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(block) = src.report_err_if(Block::dissect(reader))? {
                    assert_eq!(format!("{block};"), origin);
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    })
}

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 5000,
        ..ProptestConfig::with_cases(10)
    })]
    #[test]
    fn test_run_task(
        args in any_with::<Element>((vec![ElementRef::Function], 0))
    ) {
        reading(args.clone());
    }
}
