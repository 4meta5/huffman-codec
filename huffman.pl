test :-
	L = 'This is an example to demonstrate Huffman Coding!',
	make_code(L,C),
	sort(C,SC),
	format('Character~t   Frequency~t~40|Code~n'),
	maplist(prn, SC).
prn([N, Car, Code]):-
	format('~w :~t~w~t~40|', [Car, N]),
	forall(member(V, Code), write(V)),
	nl.

make_code(S,Dict) :-
	string_lower(S,A),
	atom_chars(A,B),
	msort(B,C),
	freq(C,D),
	sort(D,E),
	tree(E,F),
	codec(F,Dict).
is_code(Car,Dict,Code) :-
	member([_,Car,Code],Dict).
ncode(Text,Dict,Code) :-
	atom_chars(Text,A),
	maplist(=(Dict),L1),
	maplist(is_code,A,L1,R1),
	flatten(R1,Code).
head([H|_],H).
dcode(Code,Dict,R) :- dcode(Code,Dict,[],R).
dcode([],_,X,R) :- atomics_to_string(X,R).
dcode(Code,Dict,A,R) :-
	findall([X,Y],prefix(Code,Dict,X,Y),L1),
	head(L1,[CD,Car]),
	append(CD,L2,Code),
	append(A,[Car],B),
	dcode(L2,Dict,B,R).
prefix(L,Dict,Code,Car) :-
	append(Code,_,L),
	member([_,Car,Code],Dict).

/*Frequency List*/
count(X,[],[1,X],[]).
count(X,[X|Y],[N1,X],R) :- count(X,Y,[N,X],R),N1 is N+1.
count(X,[Y|T],[1,X],[Y|T]) :- dif(X,Y).
freq([],[]).
freq([X],[[1,X]]) :- !. %Cut to end recursion
freq([X|Y],[H|R]):- count(X,Y,H,R1),freq(R1,R).
tree([[X1|Y1],[X2|Y2]],R) :-
	heap([X1|Y1],[X2|Y2],R),!. %Cut to end recursion
tree([[X1|Y1],[X2|Y2]|T],R) :- 
	heap([X1|Y1],[X2|Y2],R1),
	sort([R1|T],R2),
	tree(R2,R).
heap([A|B],[C|D],R) :-
	X is A+C,
	R = [X,[A|B],[C|D]].
node([_,_,_]).
leaf([Left,Right],C,R) :-
	reverse(C,CR),
	R = [[Left,Right,CR]].
codec(X,R) :- codec(X,[],R).
codec([_,Left,Right],C,R) :-
	(node(Left) ->  codec(Left,[0 | C],X);  
        leaf(Left,[0 | C],X)),
	(node(Right) ->  codec(Right,[1 | C],Y);  
        leaf(Right,[1 | C],Y)),
	append(X,Y,R).