import 'package:test/test.dart';
import '../v032_features.dart';

// A Dart implementation of the foreign-only trait introduced in 0.32 with
// `#[uniffi::export(foreign)]` (TraitKind::ForeignOnly).
class DartGreeter implements Greeter {
  @override
  String greet(String name) => 'Hello, $name!';
}

// A Dart implementation of the `#[uniffi::export(rust, foreign)]` trait
// (TraitKind::Both), exercising the mixed Rust/foreign case.
class DartNamed implements Named {
  DartNamed(this.value);
  final String value;

  @override
  String name() => value;
}

void main() {
  ensureInitialized();

  group('0.32 HashSet / Type::Set', () {
    test('set crosses the FFI and preserves distinct values', () {
      expect(sumSet(values: {1, 2, 3, 3, 4}), 10);
      expect(sumSet(values: <int>{}), 0);
    });

    test('set round-trips through a record field', () {
      final bag = SetBag(values: {'a', 'b', 'a'});
      final echoed = echoSetBag(bag: bag);
      expect(echoed.values, equals({'a', 'b'}));
    });

    test('sets nested in a sequence render and lower correctly', () {
      final bags = [
        SetBag(values: {'a', 'b'}),
        SetBag(values: {'c'}),
      ];
      expect(setsLen(bags: bags), 3);
    });
  });

  group('0.32 Box / Type::Box', () {
    test('boxed enum variant renders as its inner type', () {
      expect(unwrapBoxed(value: EmptyMaybeBoxed()), isNull);
      expect(unwrapBoxed(value: ValueMaybeBoxed(42)), 42);
    });

    test('boxed function parameter renders as its inner type', () {
      expect(doubleBoxed(value: 21), 42);
      expect(doubleBoxed(value: -3), -6);
    });
  });

  group('0.32 trait export kinds', () {
    test('foreign-only trait accepts a Dart implementation', () {
      expect(greetWith(greeter: DartGreeter(), name: 'Dart'), 'Hello, Dart!');
    });

    test('rust, foreign trait returns the Rust implementation', () {
      final named = rustNamed();
      expect(readName(named: named), 'RustNamed');
    });

    test('rust, foreign trait accepts a Dart implementation', () {
      expect(readName(named: DartNamed('DartNamed')), 'DartNamed');
    });
  });
}
