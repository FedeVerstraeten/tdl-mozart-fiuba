local WaitSome A B C D in
   proc {WaitSome Xs}
      case Xs of H|T then
	 thread
	    {WaitOr Xs T}
	    {Browse T}
	 end
      end
   end

   thread
      {WaitSome '[|'(A [A B C D])}
      A = B * 10
   end
   B=1
end
