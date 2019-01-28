using System;
using System.Runtime.InteropServices;

namespace ConsoleApp1
{
    public class MessageHandle :SafeHandle
    {
        public MessageHandle() : base(IntPtr.Zero, true) { }
        public override bool IsInvalid => false;

        protected override bool ReleaseHandle()
        {
            Native.FreeMessage(handle);
            return true;
        }
    }
}
