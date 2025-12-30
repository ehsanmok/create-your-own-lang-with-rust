// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="crash_course.html"><strong aria-hidden="true">2.</strong> Crash Course on Computing</a></li><li class="chapter-item expanded "><a href="01_calculator/calc_intro.html"><strong aria-hidden="true">3.</strong> Calculator</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="01_calculator/grammar_lexer_parser.html"><strong aria-hidden="true">3.1.</strong> Grammar, Lexer and Parser</a></li><li class="chapter-item expanded "><a href="01_calculator/ast.html"><strong aria-hidden="true">3.2.</strong> Abstract Syntax Tree (AST) and Interpreter</a></li><li class="chapter-item expanded "><a href="01_calculator/jit_intro.html"><strong aria-hidden="true">3.3.</strong> Just-In-Time (JIT) Compiler with LLVM</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="01_calculator/basic_llvm.html"><strong aria-hidden="true">3.3.1.</strong> Basic Example</a></li><li class="chapter-item expanded "><a href="01_calculator/ast_traversal.html"><strong aria-hidden="true">3.3.2.</strong> AST Traversal Patterns</a></li></ol></li><li class="chapter-item expanded "><a href="01_calculator/vm.html"><strong aria-hidden="true">3.4.</strong> Virtual Machine (VM), Bytecode and Interpreter</a></li><li class="chapter-item expanded "><a href="01_calculator/repl.html"><strong aria-hidden="true">3.5.</strong> Read-Eval-Print Loop (REPL)</a></li><li class="chapter-item expanded "><a href="01_calculator/exercise.html"><strong aria-hidden="true">3.6.</strong> Exercises</a></li></ol></li><li class="chapter-item expanded "><a href="02_firstlang/intro.html"><strong aria-hidden="true">4.</strong> Firstlang: Your First Real Language</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="02_firstlang/syntax.html"><strong aria-hidden="true">4.1.</strong> Python-like Syntax</a></li><li class="chapter-item expanded "><a href="02_firstlang/variables.html"><strong aria-hidden="true">4.2.</strong> Variables and Assignment</a></li><li class="chapter-item expanded "><a href="02_firstlang/functions.html"><strong aria-hidden="true">4.3.</strong> Functions</a></li><li class="chapter-item expanded "><a href="02_firstlang/control_flow.html"><strong aria-hidden="true">4.4.</strong> Control Flow: If/Else and While</a></li><li class="chapter-item expanded "><a href="02_firstlang/recursion.html"><strong aria-hidden="true">4.5.</strong> Recursion</a></li><li class="chapter-item expanded "><a href="02_firstlang/repl.html"><strong aria-hidden="true">4.6.</strong> Building the REPL</a></li><li class="chapter-item expanded "><a href="02_firstlang/fibonacci.html"><strong aria-hidden="true">4.7.</strong> Computing Fibonacci</a></li></ol></li><li class="chapter-item expanded "><a href="03_secondlang/intro.html"><strong aria-hidden="true">5.</strong> Secondlang: Adding Types and Compilation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="03_secondlang/why_types.html"><strong aria-hidden="true">5.1.</strong> Why Types Matter</a></li><li class="chapter-item expanded "><a href="03_secondlang/annotations.html"><strong aria-hidden="true">5.2.</strong> Type Annotations</a></li><li class="chapter-item expanded "><a href="03_secondlang/inference.html"><strong aria-hidden="true">5.3.</strong> Type Inference</a></li><li class="chapter-item expanded "><a href="03_secondlang/optimizations.html"><strong aria-hidden="true">5.4.</strong> AST Optimizations (Visitor Pattern)</a></li><li class="chapter-item expanded "><a href="03_secondlang/ir.html"><strong aria-hidden="true">5.5.</strong> From AST to IR</a></li><li class="chapter-item expanded "><a href="03_secondlang/codegen.html"><strong aria-hidden="true">5.6.</strong> LLVM Code Generation</a></li><li class="chapter-item expanded "><a href="03_secondlang/jit_fibonacci.html"><strong aria-hidden="true">5.7.</strong> JIT Compiling Fibonacci</a></li></ol></li><li class="chapter-item expanded "><a href="resources.html"><strong aria-hidden="true">6.</strong> Resources</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
