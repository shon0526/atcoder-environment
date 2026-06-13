use ac_library::{segtree, Max, Segtree};
use proconio::{input, marker::Usize1};
define_queries! {
    enum Query: usize {
        1 => Push { pos: Usize1, x: usize},
        2 => Answer { l: Usize1, r: Usize1},
    }
}

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        backtrace_on_stack_overflow::enable()
    };
    input! {
      n: usize,
      q: usize,
      queries: [Query; q],
    }

    let mut segtree: Segtree<Max<usize>> = Segtree::new(n);
    for query in queries {
        match query {
            Query::Push { pos, x } => {
                segtree.set(pos, x);
            }
            Query::Answer { l, r } => {
                println!("{}", segtree.prod(l..r));
            }
        }
    }
}

#[macro_export]
macro_rules! define_queries {
  ($( $(#[$attr:meta])* enum $enum_name:ident : $sig:ty { $( $pattern:pat => $variant:ident $( { $($name:ident : $marker:ty $(,)?),* } )? $(,)?),* } )*) => {
    $(
      $(#[$attr])*
      enum $enum_name {
        $(
          $variant $( {
            $( $name : <$marker as proconio::source::Readable>::Output ),*
          } )?
        ),*
      }

      impl proconio::source::Readable for $enum_name {
        type Output = Self;
        fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self {
          #![allow(unreachable_patterns)]
          match <$sig as proconio::source::Readable>::read(source) {
            $(
              $pattern => $enum_name::$variant $( {
                $( $name: <$marker as proconio::source::Readable>::read(source) ),*
              } )?
            ),*
            , _ => unreachable!()
          }
        }
      }
    )*
  }
}
