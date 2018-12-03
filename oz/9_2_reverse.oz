local Reverse1 Reverse2 Rout1 Rout2 in
   fun lazy {Reverse1 S}
      fun {Rev S R}
	 case S of nil then R
	 [] X|S2 then {Rev S2 X|R} end
      end
   in {Rev S nil} end

   fun lazy {Reverse2 S}
      fun lazy {Rev S R}
	 case S of nil then R
	 [] X|S2 then {Rev S2 X|R} end
      end
   in {Rev S nil} end

   {Reverse1 [a b c] Rout1}
   {Reverse2 [a b c] Rout2}   

   {Browse 'Reverse1:'#Rout1.1}
   {Browse 'Reverse2:'#Rout2.2}
end
