use super::TableRow;
use leptos::*;
use leptos_meta::{
    provide_meta_context,
    Script,
};
use std::{
    hash::{
        DefaultHasher,
        Hash,
        Hasher,
    },
    iter::Iterator,
};
use tracing::debug;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// Intermediary type to grab the result from jquery.
    type Table;
    /// Queries the table with the given id from the DOM.
    #[wasm_bindgen(js_name = "$")]
    fn new_table(id: &str) -> Table;
    /// Enables sorting for the table with the given id.
    #[wasm_bindgen(method)]
    fn tablesort(this: &Table);
}

/// Algorithms for sorting a table column.
#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum TableSortingAlgorithm {
    /// The default, builtin sorting.
    Default,
    /// A custom float sorting algorithm.
    Float,
}

impl std::fmt::Display for TableSortingAlgorithm {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        let s = match self {
            Self::Default => "",
            Self::Float => "float",
        };
        write!(f, "{s}")
    }
}

/// A `fomantic-ui` table.
///
/// `D` defines the table data type.
/// `R` defines the row item type.
#[component]
pub fn Table<D, R>(
    /// The table data.
    #[prop(into)]
    data: MaybeSignal<D>,
    /// A list of closures defining the column heading.
    column_heading: Vec<Box<dyn Fn(NodeRef<html::Th>) -> Fragment>>,
    /// A list of closures that return the contents of each column.
    columns: Vec<Box<dyn Fn(&R) -> Fragment>>,
    /// Determines the sorting algorithm of the column.
    #[prop(optional, into)]
    column_sorting: MaybeSignal<Vec<TableSortingAlgorithm>>,
) -> impl IntoView
where
    D: IntoIterator<Item = R> + Clone + 'static,
    R: Hash + 'static,
{
    // Used for inserting custom sort algorithms via leptos-meta
    provide_meta_context();

    let heading_items = column_heading
        .into_iter()
        .enumerate()
        .map(|(idx, head)| {
            let sorting = column_sorting.clone();
            move || {
                let ref_th = create_node_ref::<html::Th>();
                let sorting_class = sorting
                    .with(|sorting_vec| {
                        sorting_vec.get(idx).map(|s| s.to_owned())
                    })
                    .map(|sort| sort.to_string())
                    .unwrap_or("".to_string());
                if !sorting_class.is_empty() {
                    ref_th.on_load(move |th| {
                        let _ = th.classes(sorting_class);
                    });
                }

                view! {
                    <th
                        node_ref=ref_th>
                        { head(ref_th) }
                    </th>
                }
            }
        })
        .collect::<Vec<_>>();

    let ref_table = create_node_ref::<leptos::html::Table>();
    let init_table = move || {
        if let Some(table) = ref_table.get() {
            let _ = table.on_mount(|_| {
                new_table("table.ui.sortable.table").tablesort();
                debug!("Initializing sortable table finished.");
            });
        }
    };

    view! {
        // add custom sort algorithms
        <Script src="/js/tablesort-custom-sort.js" defer="true"></Script>

        <table
            node_ref=ref_table
            class="ui sortable basic table">
            <thead>
                { heading_items }
            </thead>
            <tbody>
            <For
                each=move || data.get()
                key=move |item: &R| {
                    let mut hasher = DefaultHasher::new();
                    item.hash(&mut hasher);
                    hasher.finish()
                }
                children=move |item: R| {
                    let td_list = columns
                        .iter()
                        .map(|c| view! {
                            <td>
                            { c(&item) }
                            </td>
                        })
                        .collect::<Vec<_>>();
                    view! {
                        <TableRow>
                            { td_list }
                        </TableRow>
                    }
                }
            />
            </tbody>
        </table>

        { init_table }
    }
}
