local WaitSome A B C D in
   proc {WaitSome Xs}
      case Xs of H|T then
	 thread
	    {WaitOr H T}
	    {Browse Xs}
	 end
      end
   end

   thread
      {WaitSome [A B C D]}
      A = B * 10
      D = 100* C
   end

   thread
      {Delay 500}
      C=3
   end

   thread
      {Delay 1000}
      B=2
   end
end
