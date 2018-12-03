local MessageAgent in
   proc{MessageAgent Proc Port}
      local Stream in
         thread
            for Message in Stream do
	       %{Delay 2000}
	       {Proc Message}
            end
         end

        {NewPort Stream Port}
      end
   end

   local Port1 Port2 in
      {MessageAgent Browse Port1}
      {Send Port1 'Message to Port1'}
      {MessageAgent Browse Port2}
      {Send Port2 'Other message to Port2'}
      {Send Port2 222}
      {Send Port1 111}
   end

   local Port3 in
      {MessageAgent Browse Port3}
      {Send Port3 'Last message from Port3'}
      {Send Port3 333}
   end
end