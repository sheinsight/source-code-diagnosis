import generate_regular_expressions from "./get_regular_expressions.mjs"
import generate_statements from "./get_statements.mjs"
import generate_grammar from "./get_grammar.mjs"
import generate_functions from "./get_functions.mjs"
import generate_classes from "./get_classes.mjs"
import generate_operators from "./get_operators.mjs"


await Promise.all([
  // generate_statements(),
  // generate_grammar(),
  // generate_functions(),
  // generate_classes(),
  // generate_regular_expressions(),
  generate_operators()
])