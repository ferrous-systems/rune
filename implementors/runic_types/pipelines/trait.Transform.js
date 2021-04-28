(function() {var implementors = {};
implementors["fft"] = [{"text":"impl <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt;&gt; for <a class=\"struct\" href=\"fft/struct.NoiseReduction.html\" title=\"struct fft::NoiseReduction\">NoiseReduction</a>","synthetic":false,"types":["fft::noise_reduction::NoiseReduction"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"fft/struct.ShortTimeFourierTransform.html\" title=\"struct fft::ShortTimeFourierTransform\">ShortTimeFourierTransform</a>","synthetic":false,"types":["fft::stft::ShortTimeFourierTransform"]},{"text":"impl <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt;&gt; for <a class=\"struct\" href=\"fft/struct.ShortTimeFourierTransform.html\" title=\"struct fft::ShortTimeFourierTransform\">ShortTimeFourierTransform</a>","synthetic":false,"types":["fft::stft::ShortTimeFourierTransform"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">&amp;'a [</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"fft/struct.ShortTimeFourierTransform.html\" title=\"struct fft::ShortTimeFourierTransform\">ShortTimeFourierTransform</a>","synthetic":false,"types":["fft::stft::ShortTimeFourierTransform"]},{"text":"impl <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt;&gt; for <a class=\"struct\" href=\"fft/struct.Fft.html\" title=\"struct fft::Fft\">Fft</a>","synthetic":false,"types":["fft::Fft"]}];
implementors["modulo"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;T&gt; for <a class=\"struct\" href=\"modulo/struct.Modulo.html\" title=\"struct modulo::Modulo\">Modulo</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html\" title=\"trait core::ops::arith::Rem\">Rem</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["modulo::Modulo"]},{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"modulo/struct.Modulo.html\" title=\"struct modulo::Modulo\">Modulo</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html\" title=\"trait core::ops::arith::Rem\">Rem</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["modulo::Modulo"]},{"text":"impl&lt;'a, T&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"modulo/struct.Modulo.html\" title=\"struct modulo::Modulo\">Modulo</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html\" title=\"trait core::ops::arith::Rem\">Rem</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["modulo::Modulo"]}];
implementors["normalize"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"normalize/struct.Normalize.html\" title=\"struct normalize::Normalize\">Normalize</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html\" title=\"trait core::ops::arith::Div\">Div</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>","synthetic":false,"types":["normalize::Normalize"]},{"text":"impl&lt;T, const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a>T<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"normalize/struct.Normalize.html\" title=\"struct normalize::Normalize\">Normalize</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html\" title=\"trait core::ops::arith::Div\">Div</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html\" title=\"trait core::ops::arith::Sub\">Sub</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a>,&nbsp;</span>","synthetic":false,"types":["normalize::Normalize"]}];
implementors["ohv_label"] = [{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"ohv_label/struct.OhvLabel.html\" title=\"struct ohv_label::OhvLabel\">OhvLabel</a>&lt;N&gt;","synthetic":false,"types":["ohv_label::OhvLabel"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"ohv_label/struct.OhvLabel.html\" title=\"struct ohv_label::OhvLabel\">OhvLabel</a>&lt;N&gt;","synthetic":false,"types":["ohv_label::OhvLabel"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i8.html\">i8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"ohv_label/struct.OhvLabel.html\" title=\"struct ohv_label::OhvLabel\">OhvLabel</a>&lt;N&gt;","synthetic":false,"types":["ohv_label::OhvLabel"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>&gt;&gt; for <a class=\"struct\" href=\"ohv_label/struct.OhvLabel.html\" title=\"struct ohv_label::OhvLabel\">OhvLabel</a>&lt;N&gt;","synthetic":false,"types":["ohv_label::OhvLabel"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;&gt; for <a class=\"struct\" href=\"ohv_label/struct.OhvLabel.html\" title=\"struct ohv_label::OhvLabel\">OhvLabel</a>&lt;N&gt;","synthetic":false,"types":["ohv_label::OhvLabel"]},{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"struct\" href=\"runic_types/tensor/struct.Tensor.html\" title=\"struct runic_types::tensor::Tensor\">Tensor</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i8.html\">i8</a>&gt;&gt; for <a class=\"struct\" href=\"ohv_label/struct.OhvLabel.html\" title=\"struct ohv_label::OhvLabel\">OhvLabel</a>&lt;N&gt;","synthetic":false,"types":["ohv_label::OhvLabel"]}];
implementors["person_detection_agg"] = [{"text":"impl&lt;const N:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"runic_types/pipelines/trait.Transform.html\" title=\"trait runic_types::pipelines::Transform\">Transform</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">; N]</a>&gt; for <a class=\"struct\" href=\"person_detection_agg/struct.PersonDetectionAgg.html\" title=\"struct person_detection_agg::PersonDetectionAgg\">PersonDetectionAgg</a>&lt;N&gt;","synthetic":false,"types":["person_detection_agg::PersonDetectionAgg"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()