// Copyright 2016 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib;
extern crate xi_rope;
extern crate syntect;

use std::path::Path;

use crate::xi_core::ConfigTable;
use xi_plugin_lib::{mainloop, ChunkCache, Error, Plugin, View};
use xi_rope::delta::Builder as EditBuilder;
use xi_rope::interval::Interval;
use xi_rope::rope::RopeDelta;

use syntect::highlighting::StyleModifier;
use syntect::highlighting::FontStyle;

struct Column80Plugin;

impl Plugin for Column80Plugin {
    type Cache = ChunkCache;

    fn new_view(&mut self, view: &mut View<Self::Cache>) {
        eprintln!("new view {}", view.get_id());
    }

    fn did_close(&mut self, view: &View<Self::Cache>) {
        eprintln!("close view {}", view.get_id());
    }

    fn did_save(&mut self, view: &mut View<Self::Cache>, _old: Option<&Path>) {
        eprintln!("saved view {}", view.get_id());
    }

    fn config_changed(&mut self, _view: &mut View<Self::Cache>, _changes: &ConfigTable) {}

    fn update(
        &mut self,
        view: &mut View<Self::Cache>,
        delta: Option<&RopeDelta>,
        _edit_type: String,
        _author: String,
    ) {
        let style_mod = StyleModifier {
            foreground: None,
            background: None,
            font_style: Some(FontStyle::BOLD)
        };
        
     
    }
}


fn main() {
    let mut plugin = Column80Plugin;
    mainloop(&mut plugin).unwrap();
}
