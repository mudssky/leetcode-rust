[CmdletBinding()]
param(

	[Parameter(Mandatory = $True)]
	[ValidateSet( 'doc', 'test')]
	[string]$mode = 'doc'


)


switch ($mode) {
	'doc' { 
		watchexec.exe --restart --clear  --exts rs cargo doc --  --no-deps
	}
	'test' {
		cargo test --tests
	}
	Default {}
}

