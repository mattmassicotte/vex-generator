; all types

(binary_expression
	named_field: (number_literal)
	"anonymous"
	!negated_field
	(capture) @capture-name
	(one-or-more)+
	(zero-or-more)*
	(optional)?
	(
		(sibling_group_one)
		(sibling_group_two)
	)
	((quantifcation_group))*
	[(alternation_a) (alternation_b)]
	(_)
	_
	.
	(#eq? @key-name @value-name)
	(#contains @key-name "foo" "bar")
	(#any-of? @key-name "foo" "bar")
	)