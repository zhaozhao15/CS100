import copy

class program:
	def __init__(self,s):
		s='\n' + s
		s=s.replace('\t','').replace('\r','\n').replace('\n\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n')
		s=s.replace(' ','').replace('return','self.value=')
		decls = s.split('main()')[0]
		decls = decls.replace('bool','')
		main = s.split('main()')[1]
		main = main.replace('{','').replace('}','').replace('!','not ').replace('|',' or ').replace('&',' and ')
		s = decls + main
		self.ss = copy.deepcopy(main)
		self.main = copy.deepcopy(main)
		con = 'normal'
		self.sss = copy.deepcopy(s)
		try:
			for i in range(len(s)+1000):
				if s[i] == 'i' and s[i+1] == 'f':
					con = 'inif'
					index1 = s.find('\n',i)
					s = s[0:index1] + 'colon' + s[index1:]
				elif s[i] == 'w' and s[i+1] == 'h' and s[i+2] == 'i' and s[i+3] == 'l' and s[i+4] == 'e':
					con = 'inwhile'
					index2 = s.find('\n',i)
					s = s[0:index2] + 'colon' + '\n1:to be replaced' + s[index2:]
				elif s[i] == 'f' and s[i+1] == 'i':
					con = 'normal'
				elif s[i] == 'd' and s[i+1] == 'o' and s[i+2] == 'n' and s[i+3] == 'e':
					con = 'normal'
				if (con == 'inif' or con == 'inwhile') and s[i] == ':':
					s = s[0:i+1] + '    ' + s[i+1:]
		except:
			pass
		s = s.replace('\ndone','').replace('\nfi','')
		replacement = 'counter += 1;\n    if counter > 30colon\n        self.value0 = "infinite"\n        break'
		s = s.replace('to be replaced',replacement)
		try:
			for i in range(len(s)):
				if s[i] == ':':
					index = s.find('\n',i-4)
					s = s[0:index+1] + s[i+1:]
		except:
			pass
		s = s.replace('colon',':')
		self.data = s

	def evaluate(self):
		counter = 0
		self.data=self.data.strip('\n')
		self.data=self.data+'\ntry:self.value=self.value0\nexcept:pass'
		exec(self.data)
		return self.value
		
	def getCFG(self):
		ids = []
		ss = self.sss
		ss = ss.replace('\t','').replace('\r','\n').replace('\n\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n').replace('\n\n','\n')
		ss=copy.deepcopy(self.main)
		ss=ss.strip('\n')
		try:
			for i in range(len(ss)+100):
				if ss[i] == 'i' and ss[i+1] == 'f':
					index = ss.find('\n',i)
					ss = ss[0:index+1]+'dalao'+ss[index+1:]
				elif ss[i] == 'f' and ss[i+1] == 'i':
					index = ss.find('\n',i)
					ss = ss[0:index+1]+'dalao'+ss[index+1:]
				elif ss[i:i+5] == 'while':
					index1= ss.find('\n',i-4)
					index2= ss.find('\n',i)
					ss = ss[0:index1+1]+'dalao'+ss[index1+1:index2+1]+'dalao'+ss[index2+1:]		
				elif ss[i:i+4] == 'done':
					index = ss.find('\n',i)
					ss = ss[0:index+1]+'dalao'+ss[index+1:]
		except:
			pass
		data = ss.split('dalao')
		finaldata=[]
		for i in data:
			if i != '':
				finaldata.append(i)
		print(finaldata)
		number = len(finaldata)
		mylist = [[0]*number for i in range(number)]
		for i in range(number):
			if 'if' in finaldata[i]:
				mylist[i][i+1]=1
				mylist[i][i+2]=1
				mylist[i+1][i+2]=1
				try:
					mylist[i+2][i+3]=1
				except:
					pass
			elif 'while' in finaldata[i]:
				print('fffff')
				mylist[i][i+1]=1
				mylist[i][i+2]=1
				mylist[i+1][i]=1
				try:
					mylist[i-1][i]=1
				except:
					pass					
				try:
					mylist[i+2][i+3]=1
				except:
					pass
		self.main = self.main.strip('\n')
		try:
			for i in range(len(self.main)+100):
				if self.main[i] == 'i' and self.main[i+1] == 'f':
					index = self.main.find('\n',i)
					self.main = self.main[0:index+1]+'dalao'+self.main[index+1:]
				elif self.main[i] == 'f' and self.main[i+1] == 'i':
					index = self.main.find('\n',i)
					self.main = self.main[0:index+1]+'dalao'+self.main[index+1:]
				elif self.main[i:i+5] == 'while':
					index1= self.main.find('\n',i-4)
					index2= self.main.find('\n',i)
					self.main = self.main[0:index1+1]+'dalao'+self.main[index1+1:index2+1]+'dalao'+self.main[index2+1:]
				elif self.main[i:i+4] == 'done':
					index = self.main.find('\n',i)
					self.main = self.main[0:index+1]+'dalao'+self.main[index+1:]
		except:
			pass
		sub=self.main.split('dalao')
		for i in range(len(sub)):
			if sub[i] != '':
				index=sub[i].find(':')
				szq=sub[i][0:index]
				ids.append(szq)
		ids1=copy.deepcopy(ids)
		ids.sort()
		num=[i for i in range(number)]
		tmp1 = zip(ids,num)
		sxtoid=dict((y,x)for x,y in tmp1)
		tmp2=zip(num,ids1)
		idtonum=dict((y,x)for x,y in tmp2)
		result=''
		for i in range(number):
			row=idtonum[sxtoid[i]]
			for j in range(number):
				column=idtonum[sxtoid[j]]
				result = result +str(mylist[row][column])+','
				if j == number-1:
					result=result.rstrip(',')
					result=result+';'
		result=result.rstrip(';')
		result='['+result+']'
		return result